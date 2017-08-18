
use common::*;
use grammar::{GrammarToken, Pat, RawRules};
use radix_trie::{Trie, TrieCommon};
use std::rc::Rc;
use regex::Regex;

/// Rules that tell the lexing function how to split a text into tokens.
pub type LexerRules = Vec<TokenDef>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenDef {
    Named(String, GrammarToken),
    Unnamed(GrammarToken),
}

/// Finds the lexer rules defined in the given set of raw rules.
pub fn find_lexer_rules(rules: &RawRules) -> LexerRules {
    #[inline]
    fn find_tokendefs_into(pat: &Pat, tokendefs: &mut Vec<TokenDef>) {
        use grammar::Pat::*;
        match *pat {
            Rule(_) => {}
            Token(ref token) | BreakOnToken(ref token) => {
                let t = TokenDef::Unnamed(token.clone());
                if ! tokendefs.contains(&t) {
                    tokendefs.push(t);
                }
            }
            Seq(ref pats) | AnyOf(ref pats) => {
                for pat in pats {
                    find_tokendefs_into(pat, tokendefs);
                }
            }
            Cap(_, ref pat) | Opt(ref pat) | ZeroPlus(ref pat) | OnePlus(ref pat) 
            | Loop(ref pat) => {
                find_tokendefs_into(pat, tokendefs);
            }
        }
    }
    
    let mut tokendefs = Vec::new();
    // Iterate the rules in reverse insertion order, since they are inserted
    // in reversed order when reducing recursively in the Pest parser.
    for &(ref key, ref rule) in rules.iter() {
        if is_token_id(key) {
            match rule.pat {
                Pat::Token(ref token) => {
                    let t = TokenDef::Named(key.clone(), token.clone());
                    if ! tokendefs.contains(&t) {
                        tokendefs.push(t);
                    }
                }
                Pat::AnyOf(ref pats) => {
                    // do the above
                    for pat in pats {
                         if let &Pat::Token(ref token) = pat {
                             let t = TokenDef::Named(key.clone(), token.clone());
                             if ! tokendefs.contains(&t) {
                                 tokendefs.push(t);
                             }
                         } else {
                             println!("Found non-token pattern for token rule '{}': {:?}", key, pat);
                             println!("Warning: TOKEN rules must only contain a single string or regex pattern");
                         }
                    }
                }
                _ => {
                    println!("Found non-token pattern for token rule '{}': {:?}", key, rule.pat);
                    println!("Warning: TOKEN rules must only contain a single string or regex pattern");
                }
            }
        } else {
            find_tokendefs_into(&rule.pat, &mut tokendefs);
        }
    }
    tokendefs
    
}

/// A description of a small part of a source text.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The 'type' of this token; what kind of function this text part serves.
    pub name: Rc<String>,
    /// The starting byte index of this token in the source text.
    pub start: usize,
    /// The ending (excl) byte index of this token in the source text.
    pub end: usize,
}
impl Token {
    /// Creates a new token.
    pub fn new(name: Rc<String>, start: usize, end: usize) -> Token {
        Token { name, start, end }
    }
    
    /// Returns the slice of the source text that this token represents.
    pub fn slice<'a>(&self, text: &'a str) -> &'a str {
        &text[self.start..self.end]
    }
}

/// Splits the given text into tokens, based on the given set of rules.
pub fn lex(text: &str, rules: &LexerRules) -> Result<Vec<Token>, String> {
    use self::TokenDef::*;
    let mut literals = Trie::new();
    let mut regexes = Vec::new();
    for token_def in rules {
        match *token_def {
            Named(ref name, GrammarToken::Str(ref string)) => {
                literals.insert(string.clone(), Rc::new(name.clone()));
            }
            Unnamed(GrammarToken::Str(ref string)) => {
                literals.insert(string.clone(), Rc::new(string.clone()));
            }
            Named(ref name, GrammarToken::Re(ref regex)) => {
                let mut re = "^".to_string();
                re.push_str(regex);
                let reg = match Regex::new(&re) {
                    Ok(reg) => reg,
                    Err(err) => {
                        println!("Error: Could not parse regex {:?}: {:?}", regex, err);
                        continue;
                    }
                };
                regexes.push((reg, Rc::new(name.clone())));
            }
            Unnamed(GrammarToken::Re(ref regex)) => {
                let mut re = "^".to_string();
                re.push_str(regex);
                let reg = match Regex::new(&re) {
                    Ok(reg) => reg,
                    Err(err) => {
                        println!("Error: Could not parse regex {:?}: {:?}", regex, err);
                        continue;
                    }
                };
                regexes.push((reg, Rc::new(regex.clone())));
            }
        }
    }
    let mut found_tokens = Vec::new();
    let mut start = 0;
    while start < text.len() {
        //println!("{}:", start);
        let slice = &text[start..];
        if let Some(ref subtrie) = literals.get_ancestor(slice) {
            let matched = subtrie.key().unwrap();
            let token_name = subtrie.value().unwrap();
            let end = start + matched.len();
            let token = Token::new(token_name.clone(), start, end);
            //println!("-> {}: {:?}", token_name, token.slice(text));
            if ! token_name.starts_with("_") {
                found_tokens.push(token);
            }
            start = end;
        } else {
            let mut found = false;
            for &(ref regex, ref name) in &regexes {
                if let Some(m) = regex.find(slice) {
                    let s = start + m.start();
                    let e = start + m.end();
                    let token = Token::new(name.clone(), s, e);
                    //println!("-> {}: {:?}", name, token.slice(text));
                    if ! name.starts_with("_") {
                        found_tokens.push(token);
                    }
                    start = e;
                    found = true;
                    break;
                }
            }
            if ! found {
                let (line, col) = get_position(text, start);
                return Err(format!("{}:{}: Could not Lex text", line, col));
            }
        }
    }
    Ok(found_tokens)
}
