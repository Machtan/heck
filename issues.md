# Issues
- (DONE)The parser reduces quantifiers on sequences incorrectly, adding the quantifier to the last element rather than the sequence.
  Possible fix:
  Make '(' and ')' into real tokens, to that the `_quantifier` reducer won't eat
  the quantifier of a sequence due to the closing paren being a silent rule. (WORKED)
- (DONE) The capture group assigner might possibly be modifying the quantifiers on objects (+ to * or somesuch)
- 'TOKEN' rules in the rule patterns are not changed to a 'Token' pattern instead, resulting in rules that aren't found when trying to parse.
