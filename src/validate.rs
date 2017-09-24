//! Functions to validate that a grammar is logically sound.

use parser::ParserRules;
use lexer::LexerRules;
use std::rc::Rc;
use std::collections::HashSet;
use std::ops::Deref;
use grammar::{Pat, GrammarToken};

pub struct GrammarError {
    pub pos: usize, // useless atm.
    pub line: usize, // useless atm.
    pub col: usize, // useless atm.
    pub message: String,
}
impl GrammarError {
    pub fn new(pos: usize, message: String) -> GrammarError {
        GrammarError { pos, message, line: 0, col: 0 }
    }
}

/// Runs all the various validators on the given rules.
pub fn validate_rules(lexer_rules: &LexerRules, parser_rules: &ParserRules) -> Vec<GrammarError> {
    let mut lints = Vec::new();
    //eprintln!("heck: Validating whether the grammar is closed in...");
    validate_closed_in_with(parser_rules, lexer_rules, &mut |error| {
        lints.push(error);
    });
    validate_unused_tokens_with(parser_rules, lexer_rules, &mut |error| {
        lints.push(error);
    });
    validate_endless_loops_into(parser_rules, &mut lints);
    validate_left_recursion_into(parser_rules, &mut lints);
    lints
}

// TODO: Keep track of the source of the various rules, so that I can point
// out the location of errors.

/// Validates that all rules and TOKENS mentioned in the given set of parser
/// rules are actually defined, and errors if they are not.
pub fn validate_closed_in_with<F: FnMut(GrammarError)>(parser_rules: &ParserRules, lexer_rules: &LexerRules, send_error: &mut F) {
    use lexer::TokenDef;
    use grammar::GrammarToken::*;
    let mut bound_names: HashSet<String> = HashSet::new();
    bound_names.insert("EOF".to_string()); // EOF is always defined
    for (name, _) in parser_rules {
        bound_names.insert(name.clone());
    }
    for tokendef in lexer_rules {
        match *tokendef {
            TokenDef::Named(ref name, _) => {
                bound_names.insert(name.clone());
            }
            TokenDef::Unnamed(ref grammar_token) => {
                match *grammar_token {
                    Str(ref strpat) | Re(ref strpat) => {
                        bound_names.insert(strpat.clone());
                    }
                    Named(ref name) => {
                        bound_names.insert(name.deref().clone());
                    },
                }
            }
        }
    }

    fn validate_pat<F: FnMut(GrammarError)>(pat: &Pat, rule: &Rc<String>, bound: &HashSet<String>, send_error: &mut F) {
        use grammar::Pat::*;
        match *pat {
            Rule(ref name) => {
                if ! bound.contains(name) {
                    send_error(GrammarError::new(0, format!("{}: Unbound name '{}'", rule, name)));
                }
            },
            Token(ref token) | BreakOnToken(ref token) => {
                match *token {
                    GrammarToken::Named(ref name) => {
                        if ! bound.contains(name.deref()) {
                            send_error(GrammarError::new(0, format!(
                                "{}: Unbound name '{}'", rule, name
                            )));
                        }
                    }
                    _ => unreachable!("Compiled parser rules should only contain named tokens"),
                }
            },
            Seq(ref pats) | AnyOf(ref pats) => {
                for pat in pats {
                    validate_pat(pat, rule, bound, send_error)
                }
            },
            Cap(_, ref inner) | Opt(ref inner) |
            ZeroPlus(ref inner) | OnePlus(ref inner) | Loop(ref inner) => {
                validate_pat(inner, rule, bound, send_error);
            }
        }
    }
    for (_, rule) in parser_rules {
        validate_pat(&rule.pat, &rule.name, &bound_names, send_error);
    }
}


// TODO: Should I try to check rules for being unused too, and only allow
// a single 'entry point' in the grammar?
/// Validates that all named tokens are referenced by a rule.
pub fn validate_unused_tokens_with<F: FnMut(GrammarError)>(parser_rules: &ParserRules, lexer_rules: &LexerRules, send_error: &mut F) {
    use lexer::TokenDef;
    let mut tokens: HashSet<String> = HashSet::new();
    tokens.insert("EOF".to_string()); // EOF should be referenced somewhere
    for tokendef in lexer_rules {
        match *tokendef {
            TokenDef::Named(ref name, _) => {
                if ! name.starts_with("_") {
                    tokens.insert(name.clone());
                }
            }
            TokenDef::Unnamed(_) => {}
        }
    }

    fn look_for_tokens(pat: &Pat, rule: &Rc<String>, mut tokens: &mut HashSet<String>) {
        use grammar::Pat::*;
        match *pat {
            Rule(ref name) => {
                tokens.remove(name);
            },
            Token(ref token) | BreakOnToken(ref token) => {
                match *token {
                    GrammarToken::Named(ref name) => {
                        tokens.remove(name.deref());
                    }
                    _ => unreachable!("Compiled parser rules should only contain named tokens"),
                }
            },
            Seq(ref pats) | AnyOf(ref pats) => {
                for pat in pats {
                    look_for_tokens(pat, rule, &mut tokens)
                }
            },
            Cap(_, ref inner) | Opt(ref inner) |
            ZeroPlus(ref inner) | OnePlus(ref inner) | Loop(ref inner) => {
                look_for_tokens(inner, rule, &mut tokens);
            }
        }
    }
    for (_, rule) in parser_rules {
        look_for_tokens(&rule.pat, &rule.name, &mut tokens);
    }
    if ! tokens.is_empty() {
        for token in tokens {
            send_error(GrammarError::new(0, format!("Unused token: <{}>", token)));
        }
    }
}

/// Validates that uses of the 'endless loop' ('%') operator only contain
/// patterns that have at least one mandatory token read in them, meaning
/// that they always advance the token stream and thus won't loop forever.
pub fn validate_endless_loops_into(parser_rules: &ParserRules, lints: &mut Vec<GrammarError>) {
    // Find the endless loops, and validate their sequence
}

/// Validates that the grammar doesn't contain left-recursive items, so that
/// it won't get stuck in an endless loop trying to read a recursive set of
/// rules.
pub fn validate_left_recursion_into(parser_rules: &ParserRules, lints: &mut Vec<GrammarError>) {
    // Check which set of rules are reachable from the beginning of each rule,
    // and raise an error if the current rule is in it.
}

// TODO: unreachable patterns (like my greedy optional commas)
