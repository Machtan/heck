
use common::*;
use grammar::{GrammarToken, Pat, RawRules};
use trie::Trie;
use std::rc::Rc;
use regex::Regex;

/// Rules that tell the lexing function how to split a text into tokens.
pub type LexerRules = Vec<TokenDef>;

/// A token as described in the Heck grammar.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenDef {
    /// A named token, ie: 'TOKEN: "token"'
    Named(String, GrammarToken),
    /// An unnamed token, ie: 'rule: "[" expr "]"'
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
    #[inline]
    fn is_alpha(literal: &str) -> bool {
        literal.chars().all(|ch| ch.is_alphabetic())
    }
    for token_def in rules {
        match *token_def {
            Named(ref name, GrammarToken::Str(ref string)) => {
                literals.insert(string, (Rc::new(name.clone()), is_alpha(string)));
            }
            Unnamed(GrammarToken::Str(ref string)) => {
                literals.insert(string, (Rc::new(string.clone()), is_alpha(string)));
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
            _ => {
                return Err(format!("TokenDef contained a 'GrammarToken::Named' \
                    value, which shouldn't be possible"))
            }
        }
    }
    let mut found_tokens = Vec::new();
    let mut start = 0;
    while start < text.len() {
        //println!("{}:", start);
        let slice = &text[start..];
        if let Some((prefix, &(ref token_name, alpha))) = literals.find_longest_match(slice) {
            let end = start + prefix.len();
            // Check whether this is only the prefix of an identifier
            let mut is_valid = true;
            if alpha {
                if let Some(ch) = (&text[end..]).chars().next() {
                    if ch.is_alphabetic() {
                        is_valid = false;
                    }
                }
            }
            // If the prefix isn't valid, fall through to regex matching
            if is_valid {
                let token = Token::new(token_name.clone(), start, end);
                //println!("-> {}: {:?}", token_name, token.slice(text));
                if ! token_name.starts_with("_") {
                    found_tokens.push(token);
                }
                start = end;
                continue;
            }
        }
        // Regex matching
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
            return Err(format!("{}:{}: Could not Lex text (no rules matched)", line, col));
        }
        
    }
    Ok(found_tokens)
}
