//! Structures and methods to represent and parse Parses the 'heck' parser 
//! grammar.
use pest::prelude::*;

#[derive(Debug, Clone)]
pub enum Pat {
    Rule(String),
    Token(GrammarToken),
    Seq(Vec<Pat>),
    Cap(Capture, Box<Pat>),
    Opt(Box<Pat>),
    ZeroPlus(Box<Pat>),
    OnePlus(Box<Pat>),
    AnyOf(Vec<Pat>),
    Loop(Box<Pat>),
    BreakOnToken(GrammarToken),
}

#[derive(Debug, Clone)]
pub struct GrammarRule {
    pub name: String,
    pub pat: Pat,
    pub nof_captures: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GrammarToken {
    Str(String),
    Re(String),
}

#[derive(Debug, Clone)]
pub enum Capture {
    Unnamed,
    Shared(usize),
    Named(usize),
}

#[derive(Debug, Clone)]
pub enum Quantifier {
    Opt,
    ZeroPlus,
    OnePlus,
    Loop,
    BreakOnToken,
}

/// Rules as returned from the parser (only structure, no semantics).
pub type RawRules = Vec<(String, GrammarRule)>;

impl_rdp! {
    grammar! {
        rules = { (newline | ruledef)+ }
        
        rule_name   =   { plain_name | quoted_name }        
        letter      =  _{ ['a'..'z'] | ['A'..'Z'] | ["_"] }
        plain_name  =  @{ letter ~ (letter | ["-"] | ['0'..'9'])* }
        quoted_name =  @{ ["'"] ~ (!["'"] ~ any)* ~ ["'"] }
        
        newline     =  _{ ["\n"] | ["\r\n"] }
        whitespace  =  _{ [" "] | ["\t"] }
        ruledef     =   { rule_name ~ colon ~ pats_or_or ~ (newline | eoi) }
        patseq      =   { pat+ }
        patseq_nl   =   { (pat_nl ~ newline*)+ }
        pats_or_or  =   { patseq ~ (line ~ patseq)* }
        pats_or_or_nl = { newline* ~ patseq_nl ~ (newline* ~ line ~ newline* ~ patseq_nl)* ~ newline* }
        pat         =   { 
                            capture?
                            ~ (token | rule_name | ["("] ~ pats_or_or_nl ~ [")"]) 
                            ~ quantifier? 
                        }
        pat_nl      =   { 
                            capture? 
                            ~ newline*
                            ~ (token | rule_name | ["("] ~ pats_or_or_nl ~ [")"]) 
                            ~ newline*
                            ~ quantifier?
                            ~ newline*
                        }
        quantifier  =  { qmark | star | plus | exclam | modulo }
        token       =  { str_token | regex_token }
        str_token   = @{ ["\""] ~ (["\\"] ~ any | !["\""] ~ any)* ~ ["\""] }
        regex_token = @{ ["r#\""] ~ (!["\"#"] ~ any)* ~ ["\"#"] }
        number      = @{ ['0'..'9']+ }
        capture     = @{ dollar ~ (dollar* | number)? }
        
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
            (_: rule_name, name: _rule_name(), _: pats_or_or, pat: _pats_or_or()) => {
                GrammarRule { name, pat, nof_captures: 1 }
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
                println!("__pats_or_or:1");
                tail.push(pat);
                tail
            },
            (_: patseq, pat: _patseq()) => {
                println!("__pats_or_or:2");
                vec![pat]
            },
            (_: patseq_nl, pat: _patseq(), _: line, mut tail: __pats_or_or()) => {
                println!("__pats_or_or:3");
                tail.push(pat);
                tail
            },
            (_: patseq_nl, pat: _patseq()) => {
                println!("__pats_or_or:4");
                vec![pat]
            }
        }
        
        _patseq(&self) -> Pat {
            (mut rev_pats: __patseq()) => {
                println!("_patseq:1");
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
                println!("__patseq:1");
                rev_pats.push(head);
                rev_pats
            },
            (_: pat_nl, head: _pat(), mut rev_pats: __patseq()) => {
                println!("__patseq:2");
                rev_pats.push(head);
                rev_pats
            },
            () => {
                println!("__patseq:3");
                Vec::new()
            }
        }
        
        _inner_pat(&self) -> Pat {
            (_: rule_name, name: _rule_name()) => {
                println!("_inner_pat:1");
                Pat::Rule(name)
            },
            (_: token, token: _token()) => {
                println!("_inner_pat:2");
                Pat::Token(token)
            },
            (_: pats_or_or, pat: _pats_or_or()) => {
                println!("_inner_pat:3");
                pat
            },
            (_: pats_or_or_nl, pat: _pats_or_or()) => {
                println!("_inner_pat:4");
                pat
            }
        }
        
        _pat(&self) -> Pat {
            (capture: _capture(), pat: _inner_pat(), quantifier: _quantifier()) => {
                println!("_pat:1");
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
                println!("_dollars:1");
                nof_dollars + 1
            },
            () => {
                println!("_dollars:2");
                0
            }
        }
        
        _capture(&self) -> Option<Capture> {
            (_: capture, _: dollar, &num: number) => {
                println!("_capture:1");
                Some(Capture::Named(num.parse().unwrap_or(0)))
            },
            (_: capture, _: dollar, nof_dollars: _dollars()) => {
                println!("_capture:2");
                if nof_dollars > 0 {
                    Some(Capture::Unnamed)
                } else {
                    Some(Capture::Shared(nof_dollars + 1))
                }
            },
            () => {
                println!("_capture:3");
                None
            }
        }
        
        _quantifier(&self) -> Option<Quantifier> {
            (_: quantifier, quantifier: _quantifier()) => {
                println!("quantifier:1");
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
