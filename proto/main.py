"""
main.py
Heck prototyping current working file
"""
import os
import sys
from collections import namedtuple
import re

def interface(methods):
    class Interface():
        pass
    
    def genmethod(name, argnames):
        def meth(*args):
            fmt_args = ", ".join(argnames)
            raise Exception("Interface method '{}({})' was not overridden!".format(name, fmt_args))
        return meth
        
    for line in methods:
        name, *argnames = line.split()
        meth = genmethod(name, argnames)
        setattr(Interface, name, meth)
    
    return Interface


Reduce = interface([
    "scope keys",
    "key plain_or_string",
])

"""
Per-function approach?
"""

class Tokens:
    def __init__(self, tokens, source_text):
        self.index = 0
        self.tokens = tokens
        self.source_text = source_text

def read_token_or_err(token_type, tokens):
    read = advance(tokens)
    if read.type != token_type:
        error(read, [token_type])
    return read

def peek_token(tokens): # Shouldn't be necessary
    pass

def error(token, expected):
    raise Exception("Expected {}, found {!r}!".format(expected, token))

def advance(tokens):
    if tokens.index == len(tokens.tokens):
        raise Exception("No more tokens!")
    token = tokens.tokens[tokens.index]
    tokens.index += 1
    return token

def peek(tokens):
    if tokens.index == len(tokens.tokens):
        raise Exception("No more tokens!")
    return tokens.tokens[tokens.index]

def token_text(token, tokens):
    return tokens.source_text[token.start:token.end]

FIRST = {
    "key": {"STRING", "PLAIN"},
} 

Match = namedtuple("Match", [
    "type",
    "captures",
])

Token = namedtuple("Token", [
    "type",
    "start",
    "end",
])

def c_enum(members, *, doc=None, start=0):
    if doc is None:
        doc = "CEnum with members: {}".format(members)
    class CEnum:
        doc
    
    for i, name in enumerate(members, start):
        setattr(CEnum, name, i)
    
    return CEnum

"scope: '[' $key ('.' $key)* ']'"
ScopePS = c_enum([
    "BrackOpen",
    "KeyOrClose",
    "Key",
    "AfterKey",
    "Dot",
    "BrackClose",
    "End",
])

KeyPS = c_enum([
    "PlainOrString",
    "Plain",
    "String",
    "End",
], start=ScopePS.End+1)

"""
Event-based parser    
"""

def expect(token_type, token):
    if token.type != token_type:
        error(token, [token_type])

# This currently lack a way of passing the token to another state without advancing. Uh. Basically, how do I connect parsing of sub-rules correctly, so that the subparser know where to go if an optional pattern doesn't match?
ReducedPS = c_enum([
    "BrackOpen",
    "KeyOrClose",
    "DotOrClose",
    "Key",
    "End",
])
"""
Attributes should probably be on transitions?
as in, they proc as soon as you move TO the state, not when you
read a token inside it
""" 

EnterRule = namedtuple("EnterRule", [
    "name"
])
AssignMatch = namedtuple("AssignMatch", [
    "index"
])
AssignToken = namedtuple("AssignToken", [
    "index",
    "token",
])
# This doesn't really need a name, tbh. But it'll be the same space
# in a Rust enum, so let's keep it. FOR SANITY!
EndRule = namedtuple("EndRule", [
    "name"
])

def parse(tokens):
    state = ReducedPS.BrackOpen
    for token in tokens.tokens:
        if state == ReducedPS.BrackOpen:
            yield EnterRule("scope")
            expect("[", token)
            state = ReducedPS.KeyOrClose
        
        elif state == ReducedPS.KeyOrClose:
            if token.type == "]":
                yield EndRule("scope")
                state = ReducedPS.End
            elif token.type == "STRING" or token.type == "PLAIN":
                yield AssignMatch(0)
                yield EnterRule("key")
                yield AssignToken(0, token)
                yield EndRule("key")
                state = ReducedPS.DotOrClose
            else:
                error(token, ["]", "key"])
        
        elif state == ReducedPS.DotOrClose:
            if token.type == ".":
                state = ReducedPS.Key
            elif token.type == "]":
                yield EndRule("scope")
                state = ReducedPS.End
        
        elif state == ReducedPS.Key:
            if token.type == "STRING" or token.type == "PLAIN":
                yield AssignMatch(0)
                yield EnterRule("key")
                yield AssignToken(0, token)
                yield EndRule("key")
                state = ReducedPS.DotOrClose
            else:
                error(token, ["key"])
        
        elif state == ReducedPS.End:
            expect("EOF", token)
            break
    
    if state != ReducedPS.End:
        # tell what went wrong
        raise Exception("Could not finish parsing :c")

"""
RDP function-based parser with static reducer interface
"""
def parse_key(tokens, reducer):
    state = KeyPS.PlainOrString
    while True:
        if state == KeyPS.PlainOrString:
            token = advance(tokens)
            if token.type == "STRING":
                value = token
            elif token.type == "PLAIN":
                value = token
            else:
                error(token, ["STRING", "PLAIN"])
            state = KeyPS.End
        
        elif state == KeyPS.End:
            break
    
    return reducer.key(value, tokens.source_text)
                

def parse_scope(tokens, reducer):
    state = ScopePS.BrackOpen
    keys = []
    while True:
        if state == ScopePS.BrackOpen:
            read_token_or_err("[", tokens)
            token = peek(tokens)
            if token.type == "]":
                advance(tokens)
                state = ScopePS.End
            elif token.type in FIRST["key"]:
                state = ScopePS.Key
            else:
                error(token, ["]", "key"])
        
        elif state == ScopePS.Key:
            key = parse_key(tokens, reducer)
            keys.append(key)
            token = peek(tokens)
            if token.type == "]":
                advance(tokens)
                state = ScopePS.End
            elif token.type == ".":
                advance(tokens)
                state = ScopePS.Dot
            else:
                error(token, ["]", "."])
        
        elif state == ScopePS.Dot:
            token = peek(tokens)
            if token.type in FIRST["key"]:
                state = ScopePS.Key
            else:
                error(token, ["key"])
        
        elif state == ScopePS.End:
            break
    
    return reducer.scope(keys, tokens.source_text)


"""
Utils
"""
def linepos(index, text):
    lineno = 1
    line_start = 0
    for i, ch in enumerate(text):
        if i == index:
            col = index - line_start
            return (lineno, col)
        
        if ch == "\n":
            lineno += 1
            line_start = i
    
    col = index - line_start
    return (lineno, col)
            

def lex(text, simple_pats, regexes, *, ignored=None):
    if ignored is None:
        ignored = set()
    
    simple_pat_pat = re.compile("|".join(simple_pats))
    regex_pats = []
    for name, re_pat in regexes:
        regex_pats.append((name, re.compile(re_pat)))
    
    start = 0
    while start < len(text):
        rem = text[start:]
        match = simple_pat_pat.match(rem)
        if match is not None:
            end = start + match.end()
            token_type = match.group()
            if token_type not in ignored:
                token = Token(token_type, start, end)
                yield token
            
            start = end
        
        else:
            match_found = False
            for name, re_pat in regex_pats:
                match = re_pat.match(rem)
                if match is not None:
                    end = start + match.end()
                    if not name in ignored:
                        token = Token(name, start, end)
                        yield token
                    
                    start = end
                    match_found = True
                    break
            
            if not match_found:
                lineno, col = linepos(start, text)
                line = rem.splitlines()[0]
                raise Exception("{}:{}: Lexing error: No rules matched: {!r}".format(lineno, col, line))
    
    yield Token("EOF", len(text), len(text))
    
    
def token_slice(token, source_text):
    return source_text[token.start:token.end]

def clean_string(s):
    return s[1:-1]

class Red(Reduce):
    def scope(self, keys, source_text):
        return ("scope", keys)
    
    def key(self, string_or_plain, source_text) -> str:
        tt = string_or_plain.type
        if tt == "STRING":
            return clean_string(token_slice(string_or_plain, source_text))
        elif tt == "PLAIN":
            return token_slice(string_or_plain, source_text)

def _notes():
    """
    # Notes    

    It might be a LBYL parser? ie: every parsing function can assume
    that the previous state has ensured that the transition is valid,
    so the current function will just act, then figure out the next transition

    Questions: 
    - Do I validate the entry points? 
    - Do I validate every reducer?
    """      

def main():
    print("HECK!")
    SIMPLE_PATTERNS = r"\[ \] \.".split() + [" ", "\t"]
    REGEXES = [
        ("STRING", r"\"(\\.|[^\"])*\""),
        ("PLAIN", r"[a-zA-Z_][a-zA-Z_\-0-9]*"),
    ]
    IGNORED = {" ", "\t"}
    
    SOURCE_TEXT = '[ boo .   \t "hullo" ]'
    tokens = list(lex(SOURCE_TEXT, SIMPLE_PATTERNS, REGEXES, ignored=IGNORED))
    print("Tokens:")
    for token in tokens:
        print("  {!r} : {}".format(SOURCE_TEXT[token.start:token.end], token))
    
    """
    RDP parsing/reducing approach.
    """
    tokens = Tokens(tokens, SOURCE_TEXT)
    red = Red()
    scope = parse_scope(tokens, red)
    print("Scope: {}".format(scope))
    
    """
    Event-based parsing, RDP reducers.
    """
    def eparse_key(events, source_text):
        key = None
        for event in events:
            ty = type(event)
            if ty == EndRule:
                # The name is probably not needed here, only for sanity checks.
                if key.type == "PLAIN":
                    return source_text[key.start:key.end]
                else:
                    return clean_string(source_text[key.start:key.end])
            elif ty == AssignToken:
                key = event.token
    
    def eparse_scope(events, source_text):
        keys = []
        for event in events:
            ty = type(event)
            if ty == AssignMatch:
                pass
            elif ty == EnterRule:
                if event.name == "key":
                    keys.append(eparse_key(events, source_text))
            elif ty == EndRule:
                return keys
            else:
                raise Exception("Unhandled event")
    
    print("Events:")
    indent = 0
    for event in parse(tokens):
        print("  {}{}".format(" "*indent, event))
        if type(event) == EnterRule:
            indent += 2
        elif type(event) == EndRule:
            indent -= 2
    
    scope = eparse_scope(parse(tokens), SOURCE_TEXT)
    print("Scope: {}".format(scope))
    
    """
    key:            $(KEY | STRING)
    table_scope:    "[" $$key ( "." $$key )* "]"
    """
    
    """
    Event-based parsing, streaming reducer
    """
    """
    Do I have to go Events -> StaticMatch -> Ast?
    Otherwise I might want to have something like a global capture stack
    with indices to indicate where each match starts. On enter, the number
    of captures of the Rule is pushed and an index to where it starts on
    a rule stack.
    Once a scope ends, the match can be reduced... but where should it be
    added to?
    """
    
    def reduce_scope(events, source_text):
        match_stack = []
        cur_match = 
    
    events = parse(tokens)
    reduce_scope(events, SOURCE_TEXT)
    

if __name__ == '__main__':
    main()
