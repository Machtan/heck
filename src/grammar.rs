//! Structures and methods to represent and parse Parses the 'heck' parser 
//! grammar.
use pest::prelude::*;
use common::*;
use std::rc::Rc;

/// Rules as returned from the parser (only structure, no semantics).
pub type RawRules = Vec<(String, GrammarRule)>;

/// Attempts to parse a set of lexing and parsing rules from the given grammar.
pub fn parse_raw_rules(grammar: &str) -> Result<RawRules, String> {
    let mut parser = Rdp::new(StringInput::new(grammar));
    parser.rules();
    if !parser.end() {
        let (rules, strpos) = parser.expected();
        let (line, col) = get_position(grammar, strpos);
        Err(format!("{}:{}: Parsing error: expected one of rules: {:?}", line, col, rules))
    } else {
        Ok(parser.main())
    }
}

const DEBUG_REDUCER: bool = false;

#[inline(always)]
fn print(msg: &str) {
    if DEBUG_REDUCER {
        println!("{}", msg);
    }
}

#[derive(Debug, Clone)]
pub enum Pat {
    Rule(String),
    Token(GrammarToken),
    Seq(Vec<Pat>),
    Cap(CaptureInfo, Box<Pat>),
    Opt(Box<Pat>),
    ZeroPlus(Box<Pat>),
    OnePlus(Box<Pat>),
    AnyOf(Vec<Pat>),
    Loop(Box<Pat>),
    BreakOnToken(GrammarToken),
}
impl Pat {
    pub fn fmt(&self) -> String {
        let mut s = String::new();
        self.fmt_acc(&mut s);
        s
    }
    fn fmt_acc(&self, s: &mut String) {
        use self::Pat::*;
        match *self {
            Rule(ref name) => {
                s.push('\'');
                s.push_str(name);
                s.push('\'');
            }
            Token(GrammarToken::Str(ref inner)) => {
                s.push_str(&format!("{:?}", inner));
            }
            Token(GrammarToken::Re(ref inner)) => {
                s.push_str("r#");
                s.push_str(&format!("{:?}", inner));
                s.push('#');
            }
            Token(GrammarToken::Named(ref inner)) => {
                s.push('<');
                s.push_str(inner); // add '<>' ?
                s.push('>');
            }
            Seq(ref pats) => {
                let last = pats.len() - 1;
                s.push('(');
                for (i, pat) in pats.iter().enumerate() {
                    pat.fmt_acc(s);
                    if i != last {
                        s.push(' ');
                    }
                }
                s.push(')');
            }
            Cap(cap, ref pat) => {
                s.push('$');
                match cap {
                    CaptureInfo::Unnamed => {},
                    CaptureInfo::Shared(group) => {
                        for _ in 0..group+1 {
                            s.push('$');
                        }
                    }
                    CaptureInfo::Assigned(index) => {
                        s.push_str(&index.to_string());
                    }
                }
                s.push('<');
                pat.fmt_acc(s);
                s.push('>');
            }
            Opt(ref pat) => {
                pat.fmt_acc(s);
                s.push('?');
            }
            ZeroPlus(ref pat) => {
                pat.fmt_acc(s);
                s.push('*');
            }
            OnePlus(ref pat) => {
                pat.fmt_acc(s);
                s.push('+');
            }
            AnyOf(ref pats) => {
                s.push('(');
                let last = pats.len() - 1;
                for (i, pat) in pats.iter().enumerate() {
                    pat.fmt_acc(s);
                    if i != last {
                        s.push_str(" | ");
                    }
                }
                s.push(')');
            }
            Loop(ref pat) => {
                pat.fmt_acc(s);
                s.push('%');
            }
            BreakOnToken(GrammarToken::Str(ref inner)) => {
                s.push_str(&format!("{:?}", inner));
                s.push('!');
            }
            BreakOnToken(GrammarToken::Re(ref inner)) => {
                s.push_str("r#");
                s.push_str(&format!("{:?}", inner));
                s.push('#');
                s.push('!');
            }
            BreakOnToken(GrammarToken::Named(ref inner)) => {
                s.push('<');
                s.push_str(inner);
                s.push('>');
                s.push('!');
            }
        }
    }
}

/// Describes how to parse a token stream into a structural AST.
#[derive(Debug, Clone)]
pub struct GrammarRule {
    /// The name used to identify this rule in the grammar.
    pub(crate) name: String,
    /// The pattern that this rule should parse.
    pub(crate) pat: Pat,
    /// The number of capture groups in the pattern of this rule.
    /// Defaults to 1, until the captures have been assigned by
    /// 'find_and_assign_captures'.
    pub(crate) nof_captures: usize,
    /// Names for the capture group. Might be empty.
    pub(crate) capture_names: Vec<String>,
}

/// Describes a text token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GrammarToken {
    /// A fixed string, eg: "var"
    Str(String),
    /// A pattern matching this regular expression, eg: r"[a-zA-Z]+"
    Re(String),
    /// Refers to a token with the given id.
    /// Assigned by 'find_parser_rules'.
    Named(Rc<String>),
}

/// Describes what kind of capture this is.
#[derive(Debug, Clone, Copy)]
pub enum CaptureInfo {
    /// The capture is a capture group with a new id.
    Unnamed,
    /// The capture group is a shared capture group with the given id.
    /// A shared capture group is one that multiple subpatterns share and will
    /// be assigned to.
    Shared(usize),
    /// 
    /// Stage 2: This is assigned later, and not by the parser
    Assigned(usize),
}

/// Describes how many times a pattern should be parsed.
#[derive(Debug, Clone)]
pub enum Quantifier {
    /// Zero or one time.
    Opt,
    /// Zero or more times.
    ZeroPlus,
    /// One or more times.
    OnePlus,
    /// Until the loop is broken, or the internal pattern cannot be parsed.
    Loop,
    /// The pattern should break the loop it is inside, if the given token is found.
    BreakOnToken,
}

// TODO: Stricter whitespace rules wrt captures and quantifiers
impl_rdp! {
    grammar! {
        rules = { (newline | ruledef)+ }
        
        rule_name   =   { plain_name | quoted_name }        
        letter      =  _{ ['a'..'z'] | ['A'..'Z'] | ["_"] }
        plain_name  =  @{ letter ~ (letter | ["-"] | ['0'..'9'])* }
        quoted_name =  @{ ["'"] ~ (!["'"] ~ any)* ~ ["'"] }
        
        newline     =  _{ ["\n"] | ["\r\n"] }
        whitespace  =  _{ [" "] | ["\t"] }
        ruledef     =   { 
            rule_name ~ cap_names? ~ colon ~ 
            newline* ~ 
            pats_or_or ~ (newline | eoi) 
        }
        cap_names   =   { paropen ~ cap_name ~ ([","] ~ cap_name)* ~ parclose }
        cap_name    =   { rule_name } // Same rules make sense, I guess
        patseq      =   { pat+ }
        patseq_nl   =   { (pat_nl ~ newline*)+ }
        pats_or_or  =   { patseq ~ (line ~ patseq)* }
        pats_or_or_nl = { newline* ~ patseq_nl ~ (newline* ~ line ~ newline* ~ patseq_nl)* ~ newline* }
        pat         =   { 
                            capture?
                            ~ (token | rule_name | paropen ~ pats_or_or_nl ~ parclose) 
                            ~ quantifier? 
                        }
        pat_nl      =   { 
                            capture? 
                            ~ (token | rule_name | paropen ~ pats_or_or_nl ~ parclose) 
                            ~ quantifier?
                            ~ newline*
                        }
        quantifier  =  { qmark | star | plus | exclam | modulo }
        token       =  { str_token | regex_token }
        str_token   = @{ ["\""] ~ (["\\"] ~ any | !["\""] ~ any)* ~ ["\""] }
        regex_token = @{ ["r#\""] ~ (!["\"#"] ~ any)* ~ ["\"#"] }
        capture     = @{ dollar ~ dollar* }
        
        paropen     =  { ["("] }
        parclose    =  { [")"] }
        dollar      =  { ["$"] }
        star        =  { ["*"] }
        qmark       =  { ["?"] }
        plus        =  { ["+"] }
        modulo      =  { ["%"] }
        exclam      =  { ["!"] }
        colon       = _{ [":"] }
        line        =  { ["|"] }
    }
    
    process! {
        main(&self) -> RawRules {
            (_: rules, mut rev_rules: _rules()) => {
                rev_rules.reverse();
                rev_rules
            }
        }
        
        _rules(&self) -> Vec<(String, GrammarRule)> {
            (_: ruledef, rule: _ruledef(), mut rule_list: _rules()) => {
                rule_list.push((rule.name.clone(), rule));
                rule_list
            },
            () => {
                Vec::new()
            }
        }
        
        _ruledef(&self) -> GrammarRule {
            (name: _rule_name(), capture_names: _cap_names(), _: pats_or_or, pat: _pats_or_or()) => {
                GrammarRule { name, pat, capture_names, nof_captures: 1 }
            }
        }

        _cap_names(&self) -> Vec<String> {
            (_: cap_names, _: paropen, mut rev_names: __cap_names()) => {
                rev_names.reverse();
                rev_names
            },
            () => {
                Vec::new()
            }
        }
        
        __cap_names(&self) -> Vec<String> {
            (_: cap_name, name: _rule_name(), mut tail: __cap_names()) => {
                tail.push(name);
                tail
            },
            (_: parclose) => {
                Vec::new()
            }
        }
        
        _pats_or_or(&self) -> Pat {
            (mut rev_pats: __pats_or_or()) => {
                let has_one = rev_pats.len() == 1;
                if has_one {
                    rev_pats.pop().unwrap()
                } else {
                    rev_pats.reverse();
                    Pat::AnyOf(rev_pats)
                }
            }
        }
        
        __pats_or_or(&self) -> Vec<Pat> {
            (_: patseq, pat: _patseq(), _: line, mut tail: __pats_or_or()) => {
                print("__pats_or_or:1");
                tail.push(pat);
                tail
            },
            (_: patseq, pat: _patseq()) => {
                print("__pats_or_or:2");
                vec![pat]
            },
            (_: patseq_nl, pat: _patseq(), _: line, mut tail: __pats_or_or()) => {
                print("__pats_or_or:3");
                tail.push(pat);
                tail
            },
            (_: patseq_nl, pat: _patseq()) => {
                print("__pats_or_or:4");
                vec![pat]
            }
        }
        
        _patseq(&self) -> Pat {
            (mut rev_pats: __patseq()) => {
                print("_patseq:1");
                print(&format!("_patseq({:?})", rev_pats));
                let has_one = rev_pats.len() == 1;
                if has_one {
                    rev_pats.pop().unwrap()
                } else {
                    rev_pats.reverse();
                    Pat::Seq(rev_pats)
                }
            }
        }
        
        __patseq(&self) -> Vec<Pat> {
            (_: pat, head: _pat(), mut rev_pats: __patseq()) => {
                print("__patseq:1");
                rev_pats.push(head);
                rev_pats
            },
            (_: pat_nl, head: _pat(), mut rev_pats: __patseq()) => {
                print("__patseq:2");
                rev_pats.push(head);
                rev_pats
            },
            () => {
                print("__patseq:3");
                Vec::new()
            }
        }
        
        _inner_pat(&self) -> Pat {
            (_: rule_name, name: _rule_name()) => {
                print("_inner_pat:1");
                Pat::Rule(name)
            },
            (_: token, token: _token()) => {
                print("_inner_pat:2");
                Pat::Token(token)
            },
            (_: pats_or_or, pat: _pats_or_or()) => {
                print("_inner_pat:3");
                pat
            },
            (_: paropen, _: pats_or_or_nl, pat: _pats_or_or(), _: parclose) => {
                print("_inner_pat:4");
                pat
            },
            (_: pats_or_or_nl, pat: _pats_or_or()) => {
                print("_inner_pat:5");
                pat
            }
        }
        
        _pat(&self) -> Pat {
            (capture: _capture(), pat: _inner_pat(), quantifier: _quantifier()) => {
                print("_pat:1");
                print(&format!("_pat(cap: {:?}, pat: {:?}, quantifier: {:?})", capture, pat, quantifier));
                let pat = if let Some(quantifier) = quantifier {
                    match quantifier {
                        Quantifier::Opt => Pat::Opt(Box::new(pat)),
                        Quantifier::ZeroPlus => Pat::ZeroPlus(Box::new(pat)),
                        Quantifier::OnePlus => Pat::OnePlus(Box::new(pat)),
                        Quantifier::Loop => Pat::Loop(Box::new(pat)),
                        Quantifier::BreakOnToken => {
                            if let Pat::Token(token) = pat {
                                Pat::BreakOnToken(token)
                            } else {
                                // TODO: This should probably just be ignored, 
                                // then handled in a grammar validation step afterwards
                                panic!("BreakOnToken put on non-token pattern");
                            }
                        }
                    }
                } else {
                    pat
                };
                if let Some(cap) = capture {
                    Pat::Cap(cap, Box::new(pat))
                } else {
                    pat
                }
            }            
        }
        
        _dollars(&self) -> usize {
            (_: dollar, nof_dollars: _dollars()) => {
                print("_dollars:1");
                nof_dollars + 1
            },
            () => {
                print("_dollars:2");
                0
            }
        }
        
        _capture(&self) -> Option<CaptureInfo> {
            (_: capture, _: dollar, nof_dollars: _dollars()) => {
                print("_capture:2");
                if nof_dollars == 0 {
                    Some(CaptureInfo::Unnamed)
                } else {
                    Some(CaptureInfo::Shared(nof_dollars - 1))
                }
            },
            () => {
                print("_capture:3");
                None
            }
        }
        
        _quantifier(&self) -> Option<Quantifier> {
            (_: quantifier, quantifier: _quantifier()) => {
                print("quantifier:1");
                quantifier // unpack first, since it's optional
            },
            (_: qmark) => {
                Some(Quantifier::Opt)
            },
            (_: star) => {
                Some(Quantifier::ZeroPlus)
            },
            (_: plus) => {
                Some(Quantifier::OnePlus)
            },
            (_: exclam) => {
                Some(Quantifier::BreakOnToken)
            },
            (_: modulo) => {
                Some(Quantifier::Loop)
            },
            () => {
                None
            }
        }
        
        _rule_name(&self) -> String {
            (_: rule_name, rule: _rule_name()) => {
                rule
            },
            (&rule: plain_name) => {
                rule.to_string()
            },
            (&rule: quoted_name) => {
                let end_quote = rule.len() - 1;
                (&rule[1..end_quote]).to_string()
            }
        }
        
        _token(&self) -> GrammarToken {
            (&string: str_token) => {
                let last = string.len() - 1;
                let mut s = String::new();
                let mut escaped = false;
                for ch in (&string[1..last]).chars() {
                    if escaped {
                        match ch {
                            'n' => s.push('\n'),
                            'r' => s.push('\r'),
                            't' => s.push('\t'),
                            _ => s.push(ch),
                        }
                        escaped = false;
                    } else {
                        if ch == '\\' {
                            escaped = true;
                        } else {
                            s.push(ch);
                        }
                    }
                }
                GrammarToken::Str(s)
            },
            (&regex: regex_token) => {
                let last = regex.len() - 2;
                GrammarToken::Re((&regex[3..last]).to_string())
            }
        }
    }
}
