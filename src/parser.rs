
use common::*;
use std::rc::Rc;
use std::collections::HashMap;
use grammar::{Pat, CaptureInfo, GrammarToken, RawRules};
use lexer::{self, Token};
use std::iter::{self, Peekable};
use std::vec;
use std::ops::{Deref};
use captures::{CaptureType, find_and_assign_captures};

/// A named parsing pattern, with a described set of captured matches or tokens.
#[derive(Debug, Clone)]
pub struct ParserRule {
    pub(crate) name: Rc<String>,
    pub(crate) pat: Pat,
    pub(crate) captures: Vec<CaptureType>,
}

/// Rules that tells the parsing function how to combine tokens into structure.
pub type ParserRules = HashMap<String, ParserRule>;

/// Changes all UPPERCASE rules to named tokens, and all literal tokens to
/// named tokens as well.
fn assign_token_names(pat: Pat) -> Pat {
    use grammar::Pat::*;
    match pat {
        Rule(name) => {
            if is_token_id(&name) {
                Token(GrammarToken::Named(Rc::new(name)))
            } else {
                Rule(name)
            }
        }
          Token(GrammarToken::Str(s)) 
        | Token(GrammarToken::Re(s)) => {
            Token(GrammarToken::Named(Rc::new(s)))
        }
        Token(GrammarToken::Named(_)) => {
            pat // is it an err to be silent? I mean, the result is correct :p
        }
        AnyOf(pats) => {
            AnyOf(pats.into_iter().map(assign_token_names).collect())
        }
        Seq(pats) => {
            Seq(pats.into_iter().map(assign_token_names).collect())
        }
        Opt(ipat) => Opt(Box::new(assign_token_names(*ipat))),
        Cap(cap, ipat) => Cap(cap, Box::new(assign_token_names(*ipat))),
        ZeroPlus(ipat) => ZeroPlus(Box::new(assign_token_names(*ipat))),
        OnePlus(ipat) => OnePlus(Box::new(assign_token_names(*ipat))),
        Loop(ipat) => Loop(Box::new(assign_token_names(*ipat))),
          BreakOnToken(GrammarToken::Str(s)) 
        | BreakOnToken(GrammarToken::Re(s)) => {
            BreakOnToken(GrammarToken::Named(Rc::new(s)))
        }
        BreakOnToken(GrammarToken::Named(_)) => {
            pat
        }
    }
}

/// Finds and parses the parser rules in the given set of raw rules.
pub fn find_parser_rules(rules: &RawRules) -> ParserRules {
    let mut parser_rules = HashMap::new();
    for (name, rule) in rules.iter().filter_map(|&(ref k, ref v)| {
        if ! is_token_id(k) {
            Some((k.clone(), v.clone()))
        } else {
            None
        }
    }) {
        //println!("Assigning rule {:?}...", name);
        //println!("  {}", rule.pat.fmt());
        let (captures, pat_with_captures) = find_and_assign_captures(rule.pat);
        // Clean up the pat by changing tokens to named tokens, and
        let pat_with_tokens = assign_token_names(pat_with_captures);
        // TOKEN rules to named tokens as well.
        let rule = ParserRule {
            name: Rc::new(name.clone()),
            pat: pat_with_tokens,
            captures: captures,
        };
        parser_rules.insert(name, rule);
        //println!("");
    }
    parser_rules
}

/// Describes the value of a Rule or Token matched and captured by a '$' capture pattern.
#[derive(Debug, Clone)]
pub enum Capture {
    /// A single value that is always present.
    Single(Box<Match>),
    /// A value that may or may not be assigned when parsing.
    Optional(Option<Box<Match>>),
    /// A set of zero or more values, captured by repetitions.
    Multiple(Vec<Match>),
    /// A single token.
    Token(Token),
}
impl Capture {
    /// Assigns the given value to this capture, by adding its value to this capture.
    pub fn assign(&mut self, value: Match) {
        use self::Capture::*;
        match *self {
            Single(_) => {
                *self = Single(Box::new(value));
            }
            Optional(None) => {
                *self = Optional(Some(Box::new(value)));
            }
            Optional(Some(_)) => {
                panic!("Optional value assigned twice!");
            }
            Multiple(ref mut values) => {
                values.push(value);
            }
            Token(_) => {
                unreachable!();
            }
        }
    }
}

/// Describes a matched rule, including the values that were captured by it.
#[derive(Debug, Clone)]
pub struct Match {
    /// What rule was matched.
    pub rule: Rc<String>,
    /// The values that were captured.
    pub captures: Vec<Capture>,
}
impl Match {
    /// Creates a match with empty captures from a rule.
    pub fn new(rule: &ParserRule) -> Match {
        Match {
            rule: rule.name.clone(),
            captures: rule.captures.iter().map(|ct| {
                use captures::CaptureType::*;
                match *ct {
                    Single => {
                        // Use a dummy value, since this should always have been 
                        // overwritten once a parse has finished succesfully
                        Capture::Single(Box::new(Match {
                            rule: Rc::new("".to_string()),
                            captures: Vec::new(),
                        }))
                    }
                    Optional => {
                        Capture::Optional(None)
                    }
                    Multiple => {
                        Capture::Multiple(Vec::new())
                    }
                }
            }).collect()
        }
    }
    
    /// Accesses a 'single' capture value of this match at the given capture index.
    pub fn single(&self, index: usize) -> Option<&Match> {
        if index >= self.captures.len() { 
            return None;
        }
        if let Capture::Single(ref val) = self.captures[index] {
            Some(val)
        } else {
            None
        }
    }
    
    /// Accesses an 'optional' capture value of this match at the given capture index.
    pub fn optional(&self, index: usize) -> Option<Option<&Match>> {
        if index >= self.captures.len() { 
            return None;
        }
        if let Capture::Optional(ref val) = self.captures[index] {
            Some(val.as_ref().map(|b| b.deref()))
        } else {
            None
        }
    }
    
    /// Accesses a 'multiple' capture value of this match at the given capture index.
    pub fn multiple(&self, index: usize) -> Option<&Vec<Match>> {
        if index >= self.captures.len() { 
            return None;
        }
        if let Capture::Multiple(ref values) = self.captures[index] {
            Some(values)
        } else {
            None
        }
    }
    
    /// Accesses a 'token' capture value of this match at the given capture index.
    pub fn token(&self) -> Option<&Token> {
        if self.captures.len() != 1 {
            return None;
        }
        if let Capture::Token(ref token) = self.captures[0] {
            Some(token)
        } else {
            None
        }
    }
    
    pub fn fmt(&self, source: &str) -> String {
        let mut s = String::new();
        self.fmt_into(&mut s, source, 0);
        s
    }
    
    
    fn fmt_into(&self, s: &mut String, source: &str, mut indent: usize) {
        use self::Capture::*;
        #[inline]
        fn pad(s: &mut String, by: usize) {
            s.extend(iter::repeat(' ').take(by));
        }
        #[inline]
        fn is_token(mtc: &Match) -> bool {
            mtc.rule.chars().all(|ch| ch.is_uppercase())
        }
        if is_token(self) {
            s.push_str("Token<");
            s.push_str(&self.rule);
            s.push_str(">(");
            s.push_str(&format!("{:?}", self.token().unwrap().slice(source)));
            s.push_str(")>");
            return;
        }
        
        s.push_str("Match<");
        s.push_str(&self.rule);
        s.push_str(">{\n");
        indent += 2;
        for (i, cap) in self.captures.iter().enumerate() {
            match *cap {
                Single(ref mtc) => {
                    pad(s, indent);
                    s.push_str(&format!("{}  ", i));
                    mtc.fmt_into(s, source, indent);
                    s.push('\n');
                }
                Optional(ref mtc) => {
                    pad(s, indent);
                    s.push_str(&format!("{}? ", i));
                    match *mtc {
                        Some(ref mtc) => {
                            s.push_str("Some(");
                            mtc.fmt_into(s, source, indent);
                            s.push(')');
                        }
                        None => s.push_str("None"),
                    }
                    
                    s.push('\n');
                }
                Multiple(ref matches) => {
                    if matches.is_empty() {
                        pad(s, indent);
                        s.push_str(&format!("{}* []\n", i));
                    } else {
                        pad(s, indent);
                        s.push_str(&format!("{}* [\n", i));
                        for mtc in matches {
                            pad(s, indent+2);
                            mtc.fmt_into(s, source, indent+2);
                            s.push(',');
                        }
                        s.push('\n');
                        pad(s, indent);
                        s.push(']');
                        s.push('\n');
                    }
                    
                }
                Token(_) => unreachable!(),
            }
        }
        indent -= 2;
        pad(s, indent);
        s.push('}');
        
    }
}

/// A context used to return more meaningful errors when a parse fails.
struct ErrContext<'a> {
    scope: Vec<Rc<String>>,
    source_text: &'a str,
}

/// A peekable token iterator.
pub type Tokens = Peekable<vec::IntoIter<Token>>;

/// The result of a parse.
pub type ParseResult<T> = Result<T, String>;

/// Signals that the current loop or repetition should be broken out of.
#[derive(Debug)]
struct Break;

/// How a pattern would match a given token.
#[derive(Debug)]
enum ParseAction {
    /// The pattern matches and consumes the pattern in one of its branches.
    MatchesToken,
    /// The pattern doesn't match, but is optional, so the parse can continue.
    IgnoresToken,
    /// All branches of the pattern expect a different token, so the parse cannot continue.
    CannotParse,
}

/// Returns how the pattern would match a given token.
fn action_when_parsed<'a>(pat: &'a Pat, token: &Token, rules: &'a ParserRules, 
    indent: usize) -> ParseAction 
{
    use grammar::Pat::*;
    use self::ParseAction::*;
    macro_rules! prindent {
        ($($e:expr),*) => {{
            let pad = iter::repeat(" ").take(indent).collect::<String>();
            println!("{}{}", pad, format!($($e),*));
        }}
    }
    prindent!("<{}> => {})", token.name, pat.fmt());
    match *pat {
        Token(GrammarToken::Named(ref name)) => {
            let res = if &token.name == name {
                MatchesToken
            } else {
                CannotParse
            };
            prindent!("-> {:?}", res);
            res
        }
        Token(_) => { 
            panic!("Attempted parse without assigning token names"); 
        }
        BreakOnToken(GrammarToken::Named(ref name)) => {
            let res = if &token.name == name {
                MatchesToken
            } else {
                IgnoresToken
            };
            prindent!("-> {:?}", res);
            res
        }
        BreakOnToken(_) => {
            panic!("Attempted parse without assigning token names");
        }
        Rule(ref name) => {
            let pat = &rules.get(name).expect("Rule not found!").pat;
            let res = action_when_parsed(pat, token, rules, indent + 2);
            prindent!("-> {:?}", res);
            res
        }
        Seq(ref pats) => {
            for pat in pats {
                match action_when_parsed(pat, token, rules, indent+2) {
                    MatchesToken => {
                        prindent!("-> MatchesToken");
                        return MatchesToken;
                    }
                    IgnoresToken => {}
                    CannotParse => {
                        prindent!("-> CannotParse");
                        return CannotParse;
                    }
                }
            }
            prindent!("-> IsOptional");
            IgnoresToken
        }
        AnyOf(ref pats) => {
            let mut opt_found = false;
            for pat in pats {
                match action_when_parsed(pat, token, rules, indent+2) {
                    MatchesToken => {
                        prindent!("-> MatchesToken");
                        return MatchesToken;
                    }
                    // This is the correct semantics, ignoring a branch that 
                    // matches the token, if an optional branch is found first
                    IgnoresToken => opt_found = true,
                    CannotParse => {}
                }
            }
            let res = if opt_found {
                IgnoresToken
            } else {
                CannotParse
            };
            prindent!("-> {:?}", res);
            res
        }
        Opt(ref optpat) | ZeroPlus(ref optpat) => {
            let res = if let MatchesToken = action_when_parsed(optpat, token, rules, indent+2) {
                MatchesToken
            } else {
                IgnoresToken
            };
            prindent!("-> {:?}", res);
            res
        }
        OnePlus(ref ipat) | Cap(_, ref ipat) | Loop(ref ipat) => {
            let res = action_when_parsed(ipat, token, rules, indent + 2);
            prindent!("-> {:?}", res);
            res
        }
    }
}

/// Parses the given token using the given pattern, with an optional index of a capture in the capture list to assign parsed matches to.
fn parse_with_pattern<'a>(mut pat: &Pat, mut cap_idx: Option<usize>, caps: &mut Vec<Capture>, 
    rules: &ParserRules, tokens: &mut Tokens, ctx: &mut ErrContext<'a>) -> ParseResult<Option<Break>> 
{
    use grammar::Pat::*;
    use self::ParseAction::*;
    if let &Cap(CaptureInfo::Assigned(idx), ref inner_pat) = pat {
        cap_idx = Some(idx);
        pat = inner_pat;
    }
    
    #[inline]
    fn advance(tokens: &mut Tokens) -> ParseResult<lexer::Token> {
        if let Some(token) = tokens.next() {
            Ok(token)
        } else {
            Err(format!("Unexpected EOF"))
        }
    }
    
    fn error<'a>(expected: &str, found: lexer::Token, ctx: &ErrContext<'a>) -> ParseResult<Option<Break>> {
        let (line, col) = get_position(ctx.source_text, found.start);
        let scope = ctx.scope.iter().flat_map(|s| Some('.').into_iter().chain(s.chars())).skip(1).collect::<String>();
        Err(format!("{}:{}:{}: Expected {}, found {}", scope, line, col, expected, found.name))
    }
    
    fn can_consume(pat: &Pat, tokens: &mut Tokens, rules: &ParserRules) -> bool {
        use self::ParseAction::*;
        if let Some(ref peek) = tokens.peek() {
            match action_when_parsed(pat, peek, rules, 0) {
                MatchesToken => true,
                IgnoresToken | CannotParse => false,
            }
        } else {
            false
        }
    }
    
    match *pat {
        Rule(ref name) => {
            let mtc = parse_with_rule(name, rules, tokens, ctx)?;
            if let Some(idx) = cap_idx {
                caps[idx].assign(mtc);
            }
        },
        // This could technically conflict since the same namespace is used for
        // unnamed str and unnamed regex patterns.
        Token(GrammarToken::Named(ref name)) => {
            let token = advance(tokens)?;
            if &token.name == name {
                if let Some(idx) = cap_idx {
                    let mut captures = Vec::with_capacity(1);
                    captures.push(Capture::Token(token.clone()));
                    let mtc = Match { 
                        rule: name.clone(), 
                        captures
                    };
                    caps[idx].assign(mtc);
                }
            } else {
                return error(&format!("token <{}>", name), token, ctx);
            }
        }
        Token(_) => { 
            panic!("Attempted parse without assigning token names"); 
        }
        Seq(ref pats) => {
            for pat in pats {
                if let Some(Break) = parse_with_pattern(pat, cap_idx, caps, rules, tokens, ctx)? {
                    return Ok(Some(Break));
                }
            }
        }
        Opt(ref pat) => {
            if can_consume(pat, tokens, rules) {
                parse_with_pattern(pat, cap_idx, caps, rules, tokens, ctx)?;
            }
        }
        ZeroPlus(ref pat) => {
            while can_consume(pat, tokens, rules) {
                if let Some(Break) = parse_with_pattern(pat, cap_idx, caps, rules, tokens, ctx)? {
                    return Ok(Some(Break));
                }
            }
        }
        OnePlus(ref pat) => {
            if let Some(Break) = parse_with_pattern(pat, cap_idx, caps, rules, tokens, ctx)? {
                return Ok(Some(Break));
            }
            while can_consume(pat, tokens, rules) {
                if let Some(Break) = parse_with_pattern(pat, cap_idx, caps, rules, tokens, ctx)? {
                    return Ok(Some(Break));
                }
            }
        }
        AnyOf(ref pats) => {
            if tokens.peek().is_none() {
                // TODO: is this correct: The any pattern could be optional?
                return Err(format!("Unexpected EOF!")); 
            }
            let mut pat_found = false;
            // parse with the first branch that can consume the token.
            for pat in pats {
                match action_when_parsed(pat, tokens.peek().unwrap(), rules, 0) {
                    MatchesToken => {
                        if let Some(Break) = parse_with_pattern(pat, cap_idx, caps, rules, tokens, ctx)? {
                            return Ok(Some(Break));
                        }
                        pat_found = true;
                        break;
                    }
                    IgnoresToken => pat_found = true,
                    CannotParse => {}
                }
            }
            if ! pat_found {
                let mut joined = String::new();
                let last = pats.len() - 1;
                for (i, pat) in pats.iter().enumerate() {
                    joined.push_str(&pat.fmt());
                    if i != last {
                        joined.push_str(" or ");
                    }
                }
                return error(&format!("either {}", joined), tokens.next().unwrap(), ctx);
            }
        }
        Loop(ref pat) => {
            if tokens.peek().is_none() {
                return Err(format!("Unexpected EOF!"));
            }
            let start = tokens.peek().unwrap().start;
            while tokens.peek().is_some() {
                if let Some(Break) = parse_with_pattern(pat, cap_idx, caps, rules, tokens, ctx)? {
                    return Ok(Some(Break));
                }
            }
            // TODO: Is this even an error? It's probably just a grammar that 
            // should've used a repetition instead of a loop?
            let (line, col) = get_position(ctx.source_text, start);
            let scope = ctx.scope.iter().flat_map(|s| Some('.').into_iter().chain(s.chars())).skip(1).collect::<String>();
            return Err(format!("{}:{}:{}: Unclosed loop expression", scope, line, col));
        }
        BreakOnToken(GrammarToken::Named(ref name)) => {
            let should_break = tokens.peek().map_or(false, |peek| &peek.name == name);
            if should_break {
                tokens.next().unwrap();
                return Ok(Some(Break));
            } 
        }
        BreakOnToken(_) => { 
            panic!("Attempted parse without assigning token names"); 
        }
        Cap(_, _) => return Err(format!("Found a capture inside another capture!")),
    }
    
    Ok(None)
}

/// Parses the given tokens using the named rule.
fn parse_with_rule<'a>(rule: &str, rules: &ParserRules, tokens: &mut Tokens, 
    ctx: &mut ErrContext<'a>) -> ParseResult<Match> 
{
    let rule = if let Some(rule) = rules.get(rule) {
        rule
    } else {
        return Err(format!("Rule {:?} not found in the given set of rules.", rule));
    };
    let mut mtc = Match::new(&rule);
    ctx.scope.push(rule.name.clone());
    parse_with_pattern(&rule.pat, None, &mut mtc.captures, rules, tokens, ctx)?;
    let _ = ctx.scope.pop();
    Ok(mtc)
}

/// Parses the given tokens using the named 'start' rule.
pub fn parse_with_rules(start: &str, rules: &ParserRules, mut tokens: Vec<Token>, 
    source_text: &str) -> ParseResult<Match> 
{
    let eof = Token::new(Rc::new("EOF".to_string()), source_text.len(), source_text.len());
    tokens.push(eof);
    let mut tokens = tokens.into_iter().peekable();
    let mut err_ctx = ErrContext { scope: Vec::new(), source_text: source_text };
    parse_with_rule(start, rules, &mut tokens, &mut err_ctx)
}
