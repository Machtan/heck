#![recursion_limit="200"]
#![allow(unused_variables)]
#![allow(unused_macros)]
#[macro_use]
extern crate pest;
extern crate regex;
extern crate radix_trie;

mod common;
mod grammar;
mod lexer;
mod captures;

use common::*;
use grammar::*;
use pest::{StringInput, Parser};
//use std::rc::Rc;
use std::collections::{HashMap};
use lexer::{find_lexer_rules, lex, Token};
use std::iter;
use std::vec;
use std::iter::{Peekable};
use std::rc::Rc;
use captures::{CaptureType, find_and_assign_captures};
use std::ops::Deref;
//use std::slice::SliceConcatExt;

/*
const PLAIN_RULE: &str = "hello_world";
const QUOTED_RULE: &str = "'hellø wørld'";
const SIMPLE_GRAMMAR: &str = "
    rule: rule rule
";
const STRING: &str = "\"hello world\"";
const ESCAPED_QUOTE: &str = r#""hello\"world""#;
*/

const TOML_GRAMMAR: &str = r##"
_SPACE:        " "
_TAB:          "\t"
_COMMENT:    r#"#.*?"#
TRUE:         "true"
FALSE:        "false"
NEWLINE:      "\r\n" | "\n"
STRING:     r#""(?:\.|[^"])*""#
KEY:        r#"[a-zA-Z_][a-zA-Z_0-9\-]*"#
FLOAT:      r#"(?:\+|-)[0-9](?:_?[0-9])*\.(?:[0-9](?:_?[0-9])*)?"#
INT:        r#"(?:\+|-)[0-9](_?[0-9])*"#

key:    $(KEY | STRING)
endl:   NEWLINE
end:    (EOF | NEWLINE)
scope:  "[" $$key ( "." $$key )* "]" end
array:  "[" endl* "]"! $$expr "]"! ("," endl* "]"! $$expr endl* "]"!)%
expr:   $(INT | FLOAT | STRING | array | inline_table | TRUE | FALSE)
entry:  $key "=" $expr
inline_table: "{" endl* "}"! $$entry "}"! ("," endl* "}"! $$entry endl* "}"!)%


document:   (($$entry | $$scope)? end)+

"##;

const GRAMMAR: &str = r##"
scope:      "[" $$key ( "." $$key )* "]" end
document:   (($$entry | $$scope)? end)+
"##;


fn parse_rules(grammar: &str) -> Result<RawRules, String> {
    let mut parser = Rdp::new(StringInput::new(grammar));
    parser.rules();
    if !parser.end() {
        let (rules, strpos) = parser.expected();
        let (line, col) = get_position(grammar, strpos);
        Err(format!("{}:{}: Parsing error: expected one of rules: {:?}", line, col, rules))
    } else {
        println!("Queue:");
        let mut end_queue = Vec::new();
        for token in parser.queue() {
            let end = token.end;
            while end_queue.last().map_or(false, |&e| end > e) {
                end_queue.pop();
            }
            let pad = iter::repeat("  ").take(end_queue.len()).collect::<String>();
            println!("{}{:?} ({}:{})", pad, token.rule, token.start, token.end);
            end_queue.push(end);
        }
        println!("");
        Ok(parser.main())
    }
}

#[derive(Debug, Clone)]
pub struct ParserRule {
    pub name: Rc<String>,
    pub pat: Pat,
    pub captures: Vec<CaptureType>,
}

/// Rules that tells the parsing function how to combine tokens into structure.
pub type ParserRules = HashMap<String, ParserRule>;



fn find_parser_rules(rules: &RawRules) -> ParserRules {
    let mut parser_rules = HashMap::new();
    for (name, rule) in rules.iter().filter_map(|&(ref k, ref v)| {
        if ! is_token_id(k) {
            Some((k.clone(), v.clone()))
        } else {
            None
        }
    }) {
        println!("Assigning rule {:?}...", name);
        println!("  {}", rule.pat.fmt());
        let (captures, assigned_pat) = find_and_assign_captures(rule.pat);
        // Clean up the pat by changing tokens to named tokens, and
        // TOKEN rules to named tokens as well.
        let rule = ParserRule {
            name: Rc::new(name.clone()),
            pat: assigned_pat,
            captures: captures,
        };
        parser_rules.insert(name, rule);
        println!("");
    }
    parser_rules
}

#[derive(Debug, Clone)]
pub enum CaptureValue {
    Token(Token),
    Match(Box<Match>),
}
impl From<Token> for CaptureValue {
    fn from(value: Token) -> CaptureValue {
        CaptureValue::Token(value)
    }
}
impl From<Match> for CaptureValue {
    fn from(value: Match) -> CaptureValue {
        CaptureValue::Match(Box::new(value))
    }
}

#[derive(Debug, Clone)]
pub enum Capture {
    Single(CaptureValue),
    Optional(Option<CaptureValue>),
    Multiple(Vec<CaptureValue>),
}
impl Capture {
    pub fn assign<V: Into<CaptureValue>>(&mut self, value: V) {
        use self::Capture::*;
        match *self {
            Single(_) => {
                *self = Single(value.into());
            }
            Optional(None) => {
                *self = Optional(Some(value.into()));
            }
            Optional(Some(_)) => {
                panic!("Optional value assigned twice!");
            }
            Multiple(ref mut values) => {
                values.push(value.into());
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Match {
    pub rule: Rc<String>,
    pub captures: Vec<Capture>,
}
impl Match {
    pub fn new(rule: &ParserRule) -> Match {
        Match {
            rule: rule.name.clone(),
            captures: rule.captures.iter().map(|ct| {
                use captures::CaptureType::*;
                match *ct {
                    Single => {
                        // Use a dummy value, since this should always have been 
                        // overwritten once a parse has finished succesfully
                        Capture::Single(CaptureValue::Token(Token::new(Rc::new("<UNASSIGNED>".to_string()), 0, 0)))
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
}

struct ErrContext<'a> {
    scope: Vec<Rc<String>>,
    source_text: &'a str,
}

type Tokens = Peekable<vec::IntoIter<Token>>;
type ParseResult<T> = Result<T, String>;

#[derive(Debug)]
struct Break;

#[derive(Debug)]
enum ParseAction {
    MatchesToken,
    IsOptional,
    CannotParse,
}

/// Returns whether the given token can be matched or ignored by the given pattern.
fn action_when_parsed<'a>(mut pat: &'a Pat, token: &Token, rules: &'a ParserRules, 
    mut indent: usize) -> ParseAction 
{
    use grammar::Pat::*;
    use self::ParseAction::*;
    loop {
        let pad = iter::repeat(" ").take(indent).collect::<String>();
        println!("{}Action when parsed({}: <{}>)", pad, pat.fmt(), token.name);
        match *pat {
              Token(GrammarToken::Str(ref s)) 
            | Token(GrammarToken::Re(ref s)) => {
                return if token.name.deref() == s {
                    MatchesToken
                } else {
                    CannotParse
                };
            }
            BreakOnToken(GrammarToken::Str(ref s))
            | BreakOnToken(GrammarToken::Re(ref s)) => {
                return if token.name.deref() == s {
                    MatchesToken
                } else {
                    IsOptional
                };
            }
            Rule(ref name) => {
                pat = &rules.get(name).expect("Rule not found!").pat;
                indent += 2;
            }
            Seq(ref pats) => {
                for pat in pats {
                    match action_when_parsed(pat, token, rules, indent+2) {
                        MatchesToken => return MatchesToken,
                        IsOptional => {}
                        CannotParse => return CannotParse,
                    }
                }
                return IsOptional;
            }
            AnyOf(ref pats) => {
                for pat in pats {
                    match action_when_parsed(pat, token, rules, indent+2) {
                        MatchesToken => return MatchesToken,
                        // This is the correct semantics, ignoring a branch that 
                        // matches the token, if an optional branch is found first
                        IsOptional => return IsOptional,
                        CannotParse => {}
                    }
                }
                return CannotParse;
            }
            Opt(ref optpat) | ZeroPlus(ref optpat) => {
                return if let MatchesToken = action_when_parsed(optpat, token, rules, indent+2) {
                    MatchesToken
                } else {
                    IsOptional
                };
            }
            OnePlus(ref ipat) | Cap(_, ref ipat) | Loop(ref ipat) => {
                pat = ipat;
                indent += 2;
            }
        }
    }
}

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
                IsOptional | CannotParse => false,
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
        Token(GrammarToken::Str(ref s)) | Token(GrammarToken::Re(ref s)) => {
            let token = advance(tokens)?;
            if token.name.deref() == s {
                if let Some(idx) = cap_idx {
                    caps[idx].assign(token);
                }
            } else {
                return error(&format!("literal {:?}", s), token, ctx);
            }
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
            for pat in pats {
                match action_when_parsed(pat, tokens.peek().unwrap(), rules, 0) {
                    MatchesToken | IsOptional => {
                        if let Some(Break) = parse_with_pattern(pat, cap_idx, caps, rules, tokens, ctx)? {
                            return Ok(Some(Break));
                        }
                        pat_found = true;
                        break;
                    }
                    CannotParse => {}
                }
            }
            if ! pat_found {
                let mut joined = String::new();
                let last = pats.len() - 1;
                for (i, pat) in pats.iter().enumerate() {
                    joined.push('\'');
                    joined.push_str(&pat.fmt());
                    joined.push('\'');
                    if i != last {
                        joined.push_str(" or ");
                    }
                }
                return error(&format!("Either {}", joined), tokens.next().unwrap(), ctx);
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
        BreakOnToken(GrammarToken::Str(ref s)) | BreakOnToken(GrammarToken::Re(ref s)) => {
            if let Some(peek) = tokens.peek() {
                if peek.name.deref() == s {
                    return Ok(Some(Break));
                }
            }
        }
        Cap(_, _) => return Err(format!("Found a capture inside another capture!")),
    }
    
    Ok(None)
}

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

fn parse_with_rules(start: &str, rules: &ParserRules, mut tokens: Vec<Token>, 
    source_text: &str) -> ParseResult<Match> 
{
    let eof = Token::new(Rc::new("EOF".to_string()), source_text.len(), source_text.len());
    tokens.push(eof);
    let mut tokens = tokens.into_iter().peekable();
    let mut err_ctx = ErrContext { scope: Vec::new(), source_text: source_text };
    parse_with_rule(start, rules, &mut tokens, &mut err_ctx)
}

const TOML_FILE: &str = include_str!("../Cargo.toml");

/*
What needs to be done for this to work well
First, parsing the grammar should yield:
  1. An in-order list of token definitions
       This should also include the implicitly defined tokens.
  2. An out-of-order list of rule definitions

Tokens ids should probably be changed into ints, with a list for name lookup.
  This should be done inside the 'Rule' struct too.

Then, the grammar should be validated:
- All tokens and rules referred to should be defined. (closed in)
- Loops should have a break inside them. 
    And something else that reads?

Then the parsing can begin:
  - pattern_can_bypass
  - pattern_can_consume
  - parse_with_rules
*/



macro_rules! parse_and_print {
    ($grammar:expr, $grammar_rule:ident, $reducer:ident) => {
        {
            println!("----------------------------------------------------");
            let mut parser = Rdp::new(StringInput::new($grammar));
            parser.$grammar_rule();
            println!("Queue:");
            for token in parser.queue() {
                let (line, col) = get_position($grammar, token.start);
                println!("  {}:{}: {:?}: {:?}", line, col, token.rule, &$grammar[token.start..token.end]);
            }
            println!("");
            if !parser.end() {
                let (rules, strpos) = parser.expected();
                let (line, col) = get_position($grammar, strpos);
                println!("Parsing error at pos {} ({}:{})", strpos, line, col);
                println!("{}:{}: Parsing error: expected rules: {:?}", line, col, rules);
            } else {
                println!("Reduced: {:#?}", parser.$reducer());
            }
            println!("----------------------------------------------------");
            println!("");
        }
    }
}

fn main() {
    /*parse_and_print!(PLAIN_RULE, rule_name, _rule_name);
    parse_and_print!(QUOTED_RULE, rule_name, _rule_name);
    parse_and_print!(SIMPLE_GRAMMAR, rules, main);
    parse_and_print!(STRING, str_token, _token);
    parse_and_print!(ESCAPED_QUOTE, str_token, _token);
    parse_and_print!(TOML_GRAMMAR, rules, main);*/
    let raw_rules = parse_rules(TOML_GRAMMAR).expect("Could not parse TOML grammar");
    let lexer_rules = find_lexer_rules(&raw_rules);
    println!("Token definitions:");
    for def in &lexer_rules {
        println!("  {:?}", def);
    }
    let tokens = lex(TOML_FILE, &lexer_rules).expect("Lexing failed D:");
    //println!("Found tokens:");
    //for token in &tokens {
    //    println!("  {}: {:?}", token.name, token.slice(TOML_FILE));
    //}
    let parser_rules = find_parser_rules(&raw_rules);
    println!("");
    println!("Parser rules:");
    for (name, rule) in &parser_rules {
        if rule.captures.is_empty() {
            println!("{}()", name);
            continue;
        }
        print!("{}(", name);
        let last = rule.captures.len() - 1;
        for (i, &cap) in rule.captures.iter().enumerate() {
            match cap {
                CaptureType::Single => print!("arg"),
                CaptureType::Optional => print!("arg?"),
                CaptureType::Multiple => print!("[args]"),
            }
            if i != last {
                print!(", ");
            }
        }
        println!(")");
        println!("  -> {}", rule.pat.fmt());
    }
    let mtc = parse_with_rules("document", &parser_rules, tokens, TOML_FILE)
        .expect("Could not parse TOML document");
    println!("Match: {:#?}", mtc);
}
