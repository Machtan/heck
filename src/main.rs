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
mod parser;

use common::*;
use grammar::*;
use pest::{StringInput, Parser};
//use std::rc::Rc;
use lexer::{find_lexer_rules, lex};
use std::iter;
use captures::{CaptureType};
use parser::{find_parser_rules, parse_with_rules, Match};
use std::collections::HashMap;
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
scope:  "[" $$key ( "." $$key )* "]"
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

type TomlTable = HashMap<String, TomlValue>;

#[derive(Debug, Clone)]
pub enum TomlValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Array(Vec<TomlValue>),
    Table(TomlTable),
}

type TomlResult<T> = Result<T, String>;

fn with_scope<'a, F: FnOnce(&'a mut TomlTable), I: Iterator<Item=S>, S: ToString> (table: &'a mut TomlTable, mut path: I, f: F) {
    match path.next() {
        None => {
            f(table)
        }
        Some(key) => {
            let next = table.entry(key.to_string()).or_insert_with(|| TomlValue::Table(TomlTable::new()));
            if let &mut TomlValue::Table(ref mut table) = next {
                with_scope(table, path, f);
            } else {
                panic!("Scope element is not table :c");
            }
        }
    }
}

// TODO: if I want it to parse 'real' toml :p
fn clean_string(s: &str) -> String {
    s.to_string()
}

fn reduce_key(m: &Match, source: &str) -> String {
    let m = m.single(0).unwrap();
    match m.rule.as_str() {
        "STRING" => clean_string(m.token().unwrap().slice(source)),
        "KEY" => m.token().unwrap().slice(source).to_string(),
        _ => unreachable!(),
    }
}

fn reduce_inline_table(m: &Match, source: &str) -> TomlResult<TomlValue> {
    let mut table = TomlTable::new();
    for res in m.multiple(0).unwrap().into_iter().map(|m| reduce_entry(m, source)) {
        let (k, v) = res?;
        table.insert(k, v);
        // TODO: ensure keys only added once.
    }
    Ok(TomlValue::Table(table))
}

fn reduce_array(m: &Match, source: &str) -> TomlResult<TomlValue> {
    let mut arr = Vec::new();
    for res in m.multiple(0).unwrap().into_iter().map(|m| reduce_expr(m, source)) {
        arr.push(res?);
    }
    Ok(TomlValue::Array(arr))
}

fn reduce_expr(m: &Match, source: &str) -> TomlResult<TomlValue> {
    let m = m.single(0).unwrap();
    Ok(match m.rule.as_str() {
        "INT" => {
            TomlValue::Int(m.token().unwrap().slice(source).parse().expect("Invalid int :c"))
        }
        "FLOAT" => {
            TomlValue::Float(m.token().unwrap().slice(source).parse().expect("Invalid float :c"))
        }
        "STRING" => {
            TomlValue::Str(clean_string(m.token().unwrap().slice(source)))
        }
        "array" => reduce_array(m, source)?,
        "inline_table" => reduce_inline_table(m, source)?,
        "TRUE" => TomlValue::Bool(true),
        "FALSE" => TomlValue::Bool(false),
        _ => unreachable!(),
    })
}

fn reduce_entry(m: &Match, source: &str) -> TomlResult<(String, TomlValue)> {
    let key = reduce_key(m.single(0).unwrap(), source);
    let value = reduce_expr(m.single(1).unwrap(), source)?;
    Ok((key, value))
}

fn reduce_scope(m: &Match, source: &str) -> Vec<String> {
    m.multiple(0).unwrap().into_iter().map(|m| reduce_key(m, source)).collect()
}

fn reduce_document(m: &Match, source: &str) -> TomlResult<TomlTable> {
    let mut table = HashMap::new();
    let mut scope = Vec::new();
    for tlitem in m.multiple(0).unwrap() {
        match tlitem.rule.as_str() {
            "scope" => {
                scope = reduce_scope(tlitem, source);
            }
            "entry" => {
                let (key, val) = reduce_entry(tlitem, source)?;
                with_scope(&mut table, scope.iter(), |table| {
                    table.insert(key, val);
                });
            }
            _ => unreachable!(),
        }
    }
    Ok(table)
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
    let document = reduce_document(&mtc, TOML_FILE).expect("Invalid document");
    println!("Document: {:#?}", document);
}
