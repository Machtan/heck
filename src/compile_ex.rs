
const token_names: &[str; 5] = &[
    "KEY",
    "STRING",
    "[",
    "]",
    ".",
];

#[repr(usize)]
enum TokenId {
    KEY = 0,
    STRING = 1,
    Unnamed0 = 2,
    Unnamed1 = 3,
    Unnamed2 = 4,
}

const rule_names: &[str; 2] = &[
    "scope",
    "key",
];

#[repr(usize)]
enum RuleId {
    Scope = 0,
    Key = 1,
}

/*
table_scope:  "[" $$key ( "." $$key )* "]"
key:    $(KEY | STRING)
*/

// rule: for scoping
// accept: is this a valid state to end in
// capture_index: capture the token matched in this state into something?
type StateData = (Option<RuleId>, bool, Option<usize>);

const state_data: &[StateData; 7] = &[
    (Some(RuleId::Scope), )
];

#[repr(usize)]
enum ParseState {
    Scope0 = 0, // "["
    Scope1 = 1, // "."
    Scope2 = 2, // "]"
    Key0 = 3, // KEY
    Key1 = 4, // STRING
}



pub struct ScopeMatch {
    pub keys: Vec<KeyMatch>,
}

#[inline(always)]
fn parse_scope(tokens: Vec<Token>, state: &mut ParseState, ) -> Result<ScopeMatch, String> {
    state = 
}

const START: ParseState = ParseState::Scope0;

fn parse(tokens: Vec<Token>) -> ScopeMatch {
    let mut match_stack = Vec::new();
    let mut scope = Vec::new();
    let mut state = START;
    scope.push(START);
}

pub trait Reduce {
    type Err;
    type Scope;
    type Key;
    
    fn key(token: Token) -> Result<Self::Key, Self::Err>;
    // Give it an iterator instead and do some generating work in the parser?
    // Or have 'stages' ? (Init, Add, End)
    fn scope(keys: Vec<Self::Key>) -> Result<Self::Scope, Self::Err>;
}

/*
# Required to finish this compilation thing:

- Find and save what kinds of tokens/matches can be held in each match type
  - This should be simple enough to do in the capture group assignment function
- Allow some way of naming the capture groups in the grammar
  - This could be a syntax like 'scope(keys): <pattern>'
  - Naming should be optional, but might be useful for the interpreted version too :)


# Problems

- How do I parse most efficiently? Do I need to measure this? (I probably do)
  - A big loop might work with homogenic matches, but one of the things I wanted was for matches to be statically safe. So am I relegated to individual parsing functions afterall?
  - Should I try to make reduction part of it afterall? I could maybe do like archan did with the trait to specify static functions to reduce stuff... it would look fairly complex, though.


*/
