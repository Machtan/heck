# Overview
## Terminology
A rule is a parsing rule, naming and describing a syntactical sub-element.
Type: Rule

A syntax pattern is a regex-like pattern, describing a sequence of tokens and rules to expect in a source file. It also includes a set of capture markers, that specify which parts of the pattern to save to the structural AST.
Type: Pat

A capture group is a group of subpatterns in a rule with the same capture group index, which will then contain the values found when parsing these subpatterns inside a rule.
Type: CaptureType

A structural AST is a tree that contains a representation of a subset of the syntax elements in a source file.
Type: Match

A reducer is a function that transforms a structural AST to a semantical AST.
Type: typically fn(m: &Match, source_text: &str) -> Result<SemAst, Err>



## Data transformations
RawRules: Vec<(String, GrammarRule)>
LexerRules: Vec<TokenDef>
ParserRules: HashMap<String, ParserRule>

parse grammar: Grammar -> RawRules
find lexer rules: RawRules -> LexerRules
find parser rules: RawRules -> ParserRules
find and assign captures: Pat -> CaptureTypes, Pat


