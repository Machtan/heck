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
use captures::{ParserRule, CaptureType, find_and_assign_captures};

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
        let parser_rule = find_and_assign_captures(rule.pat);
        parser_rules.insert(name, parser_rule);
        println!("");
    }
    parser_rules
}


fn parse_with_rules(tokens: &Vec<Token>, rules: &ParserRules, start: &str) {

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
    parse_with_rules(&tokens, &parser_rules, "document");
}
