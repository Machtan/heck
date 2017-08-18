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

use common::*;
use grammar::*;
use pest::{StringInput, Parser};
//use std::rc::Rc;
use std::collections::{HashMap};
use lexer::{find_lexer_rules, lex, Token};
use std::cmp;
use std::iter;

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

derp: ($KEY?)+
herp: ($$KEY?)+
burp: ($KEY|$KEY)+
lurp: (($KEY|$KEY)? )+
bob:  ($$KEY?)+
blub: (($$KEY|$$KEY)? )+
birb: (($$KEY)? end)+
burb: (end ($$KEY)? end)+

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

/// Rules that tells the parsing function how to combine tokens into structure.
pub type ParserRules = HashMap<String, ParserRule>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CaptureType {
    Single,
    Optional,
    Multiple,
}

#[derive(Debug, Clone)]
pub struct ParserRule {
    pub pat: Pat,
    pub captures: Vec<CaptureType>,
}

#[derive(Debug, Clone)]
struct CaptureState {
    pub single_ids: Vec<usize>,
    pub shared_ids: Vec<usize>, // [#dollars-1 -> idx]
    pub capture_types: Vec<CaptureType>,
}
impl CaptureState {
    fn assign(&mut self, group: Option<usize>, context: CaptureContext) -> usize {
        println!("CaptureState.assign({:?}, {:?})", group, context);
        use self::CaptureContext::*;
        if let Some(group) = group {
            // The group is not the next shared one, and is much larger
            if group > self.shared_ids.len() {
                panic!("Shared id '{}' assigned before the previous one.", group)
            
            // The group is the next shared group
            } else if group == self.shared_ids.len() {
                let id = self.capture_types.len();
                self.shared_ids.push(id);
                let ty = match context {
                    Free => CaptureType::Single,
                    Optional => CaptureType::Optional,
                    Repetition => CaptureType::Multiple,
                };
                self.capture_types.push(ty);
                id
            
            // The group is an already assigned shared group
            } else {
                self.capture_types[group] = CaptureType::Multiple;
                group
            }
        } else {
            let id = self.capture_types.len();
            self.single_ids.push(id);
            let ty = match context {
                Free => CaptureType::Single,
                Optional => CaptureType::Optional,
                Repetition => CaptureType::Multiple,
            };
            self.capture_types.push(ty);
            id
        }
    }
}

fn combine_caps(indices: &mut Vec<usize>, types: &mut Vec<CaptureType>,
    oindices: &[usize], otypes: &[CaptureType]) 
{
    use self::CaptureType::*;
    let n1 = indices.len();
    let n2 = oindices.len();
    for i in 0..cmp::min(n1, n2) {
        match otypes[i] {
            Optional => {
                if let Single = types[i] {
                    types[i] = Optional;
                }
            }
            Multiple => {
                types[i] = Multiple;
            }
            _ => {}
        }
    }
    if n1 < n2 {
        for i in n1..n2 {
            indices.push(i);
            match otypes[i] {
                Single | Optional => {
                    types.push(Optional);
                }
                Multiple => {
                    types.push(Multiple);
                }
            }
        }
    } else if n1 > n2 {
        for i in n2..n1 {
            if let Single = types[i] {
                types[i] = Optional;
            }
        }
    }
}

fn reorder_capture_indices(pat: Pat, good: &CaptureState, bad: &CaptureState) -> Pat {
    assert!(good.capture_types.len() >= bad.capture_types.len());
    let mut map = vec![0, good.capture_types.len()];
    for (&bad_idx, &good_idx) in bad.single_ids.iter().zip(&good.single_ids) {
        map[bad_idx] = good_idx;
    }
    for (&bad_idx, &good_idx) in bad.shared_ids.iter().zip(&good.shared_ids) {
        map[bad_idx] = good_idx;
    }
    fn inner(pat: Pat, map: &[usize]) -> Pat {
        use grammar::Pat::*;
        match pat {
            Rule(_) | Token(_) | BreakOnToken(_) => pat,
            Seq(pats) => {
                Seq(pats.into_iter().map(|p| inner(p, map)).collect())
            }
            Cap(Capture::Assigned(idx), boxed) => {
                Cap(Capture::Assigned(map[idx]), Box::new(inner(*boxed, map)))
            }
            Cap(_, _) => panic!("NONONONONONO"),
            Opt(boxed) | ZeroPlus(boxed) | OnePlus(boxed) | Loop(boxed) => {
                Opt(Box::new(inner(*boxed, map)))
            }
            AnyOf(pats) => {
                AnyOf(pats.into_iter().map(|p| inner(p, map)).collect())
            }
        }
    }
    inner(pat, &map)
}

#[derive(Debug, Clone, Copy)]
enum CaptureContext {
    Free,
    Optional,
    Repetition,
}

fn find_and_assign_captures(pat: Pat) -> ParserRule {
    fn is_single(pat: &Pat) -> bool {
        match *pat {
            Pat::Token(_) | Pat::Rule(_) => true,
            _ => false,
        }
    }
    
    fn inner(pat: Pat, context: CaptureContext, state: &mut CaptureState) -> Pat {
        use grammar::Pat::*;
        use self::CaptureContext::*;
        match pat {
            Seq(pats) => {
                Seq(pats.into_iter().map(|p| inner(p, context, state)).collect())
            }
            Cap(captype, boxed) => {
                let group = match captype {
                    Capture::Unnamed => None,
                    Capture::Shared(idx) => Some(idx),
                    Capture::Assigned(_) => {
                        panic!("find_and_assign_captures called on a pattern whose captures were already assigned!");
                    }
                };
                let inner_context = match *boxed {
                    Token(_) | Rule(_) => Free,
                    Seq(_) | ZeroPlus(_) | OnePlus(_) | Loop(_) => Repetition,
                    Cap(_, _) => {
                        panic!("Cannot have a capture just inside a capture :/");
                    }
                    Opt(ref opt_pat) => {
                        // Find out what's inside it...
                        if is_single(opt_pat) {
                            Optional
                        } else {
                            Repetition
                        }
                    }
                    AnyOf(ref pats) => {
                        if pats.iter().all(is_single) {
                            Free
                        } else {
                            Repetition
                        }
                    },
                    BreakOnToken(_) => {
                        panic!("Cannot capture 'break on token ( \"token\"! )' pattern!");
                    }
                };
                let actual = match (context, inner_context) {
                    (Repetition, _) => Repetition,
                    (Optional, Free) => Optional,
                    (Optional, Optional) => Optional,
                    (Optional, Repetition) => Repetition,
                    (Free, other) => other,
                };
                
                let id = state.assign(group, actual);
                let new_cap_pat = inner(*boxed, context, state);
                Cap(Capture::Assigned(id), Box::new(new_cap_pat))
            }
            Opt(boxed) => {
                Opt(Box::new(if let CaptureContext::Repetition = context {
                    inner(*boxed, context, state)
                } else {
                    inner(*boxed, CaptureContext::Optional, state)
                }))
            }
            ZeroPlus(boxed) => {
                ZeroPlus(Box::new(inner(*boxed, CaptureContext::Repetition, state)))
            }
            OnePlus(boxed) => {
                OnePlus(Box::new(inner(*boxed, CaptureContext::Repetition, state)))
            }
            Loop(boxed) => {
                Loop(Box::new(inner(*boxed, CaptureContext::Repetition, state)))
            }
            AnyOf(mut pats) => {
                let pre_state = state.clone();
                let mut drained = pats.drain(..);
                let first = inner(drained.next().unwrap(), context, state);
                let mut assigned_pats = vec![first];
                for pat in drained {
                    let mut pat_state = pre_state.clone();
                    let assigned = inner(pat, context, &mut pat_state);
                    
                    
                    // Combine both capture groups, then reorder the indices of the branch pattern
                    // so that they match up.
                    // ie, if path 1 is [0shared, 1single, 2shared, 3single]
                    // and path 2 is [0single, 1shared, 2shared, 3single]
                    // path 2 needs to be changed so that the indices point to
                    // the corresponding single/shared of path 1
                    
                    combine_caps(
                        &mut state.single_ids, 
                        &mut state.capture_types,
                        &pat_state.single_ids, 
                        &pat_state.capture_types
                    );
                    combine_caps(
                        &mut state.shared_ids, 
                        &mut state.capture_types,
                        &pat_state.shared_ids, 
                        &pat_state.capture_types
                    );
                    
                    let mapped = reorder_capture_indices(assigned, state, &pat_state);
                    assigned_pats.push(mapped);
                }
                AnyOf(assigned_pats)
            }
            Token(_) | Rule(_) | BreakOnToken(_) => pat,
        }
    }
    let mut state = CaptureState { 
        single_ids: Vec::new(),
        shared_ids: Vec::new(),
        capture_types: Vec::new(),
    };
    let context = CaptureContext::Free;
    let assigned_pat = inner(pat, context, &mut state);
    ParserRule {
        pat: assigned_pat,
        captures: state.capture_types,
    }
}

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
        /*let parser_rule = find_and_assign_captures(rule.pat);
        parser_rules.insert(name, parser_rule);*/
        println!("");
    }
    parser_rules
}


fn parse_with_rules(tokens: &Vec<Token>, rules: &ParserRules, start: &str) {}

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
    let raw_rules = parse_rules(GRAMMAR).expect("Could not parse TOML grammar");
    let lexer_rules = find_lexer_rules(&raw_rules);
    println!("Token definitions:");
    for def in &lexer_rules {
        println!("  {:?}", def);
    }
    /*let tokens = lex(TOML_FILE, &lexer_rules).expect("Lexing failed D:");
    println!("Found tokens:");
    for token in &tokens {
        println!("  {}: {:?}", token.name, token.slice(TOML_FILE));
    }*/
    let parser_rules = find_parser_rules(&raw_rules);
    println!("");
    println!("Parser rules:");
    for (name, rule) in &parser_rules {
        println!("{}({:?})", name, rule.captures);
        println!("  pat: {}", rule.pat.fmt());
    }
    //parse_with_rules(&tokens, &parser_rules, "document");
}
