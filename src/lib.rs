#![recursion_limit="200"]
#[macro_use]
extern crate pest;
extern crate regex;

mod common;
mod grammar;
pub mod lexer;
mod captures;
mod parser;
mod trie;
mod validate;

pub use grammar::{GrammarRule, RawRules, GrammarToken, parse_raw_rules};
pub use lexer::{Token, TokenDef, find_lexer_rules, lex, LexerRules};
pub use captures::{CaptureType};
pub use parser::{find_parser_rules, parse_with_rules, Match, ParserRules};
pub use validate::{
    validate_closed_in_with, 
    validate_endless_loops_into, 
    validate_left_recursion_into,
    validate_rules
};

/*
Ideas for API design:
What do I expose, and what would be convenient?
- When I make a compiled backend too, it would make sense to have the 'parse raw rules' as a separate step, so that they can both share it.
- How many steps should I enforce that users go through? error messages might be easier to understand with the steps more explicit, but it might be more inconvenient to have to manage multiple errors and more calls.
- How many of the internal types should I expose? Messing with patterns can result in bad matching, so it's not really *safe*, I guess... but it might be nice to use if one wants to do something advanced. For now I should probably not expose it.

*/

pub fn lex_and_parse_with_grammar(text: &str, grammar: &str, start_with_rule: &str) 
    -> Result<Match, String> 
{
    let raw_rules = parse_raw_rules(grammar)?;
    let lexer_rules = find_lexer_rules(&raw_rules);
    let parser_rules = find_parser_rules(&raw_rules);
    let errors = validate_rules(&lexer_rules, &parser_rules);
    if errors.len() != 0 {
        let mut s = String::new();
        s.push_str("Grammar errors:");
        for (i, err) in errors.into_iter().enumerate() {
            s.push_str(&format!("\n  {}: {}", i, err.message));
        }
        return Err(s);
    }
    let tokens = lex(text, &lexer_rules)?;
    parse_with_rules(start_with_rule, &parser_rules, tokens, text)
}
