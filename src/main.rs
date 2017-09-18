extern crate heck;
extern crate argonaut;

use std::env;
use argonaut::{ArgDef, parse, ParseError, help_arg, version_arg};
use std::process;
use std::io::{self, Read, Write};
use std::error::Error;
use heck::{parse_raw_rules, find_lexer_rules, find_parser_rules, lex, parse_with_rules, LexerRules, ParserRules};
use std::path::Path;
use std::fs::File;

fn main() {
    // Properly set exit codes after the program has cleaned up.
    if let Some(exit_code) = argonaut_main() {
        process::exit(exit_code);
    }
}

pub fn try_parse(source: &str, lexer_rules: &LexerRules, parser_rules: &ParserRules) -> Option<i32> {
    println!("Parsing...");
    let tokens = match lex(source, &lexer_rules) {
        Ok(tokens) => tokens,
        Err(err) => {
            println!("{}", err);
            return Some(2);
        }
    };
    println!("Parsed tokens:");
    for token in &tokens {
        println!("  {:?}", token);
    }

    let mtc = match parse_with_rules("program", &parser_rules, tokens, source) {
        Ok(mtc) => mtc,
        Err(err) => {
            println!("Could not parse file: {}", err);
            return Some(3);
        }
    };
    println!("Found match: {}", mtc.fmt(source));
    None
}

pub fn run_prompt(grammar: &str) -> Option<i32> {
    let raw_rules = match parse_raw_rules(grammar) {
        Ok(rules) => rules,
        Err(err) => {
            println!("Could not parse grammar: {}", err);
            return Some(1);
        }
    };
    let lexer_rules = find_lexer_rules(&raw_rules);
    let parser_rules = find_parser_rules(&raw_rules);

    println!("Welcome to the heck prompt. 
Type text in the current grammar to let heck try to parse it.
Type 'quit' to quit.");

    let mut input = String::new();
    let mut prompt = "> ";
    loop {
        input.clear();
        print!("{}", prompt);
        let _ = io::stdout().flush();
        if let Err(err) = io::stdin().read_line(&mut input) {
            println!("Could not read from stdin: {}", err.description());
            continue;
        }
        if input.trim() == "quit" {
            break;
        }
        match try_parse(&input, &lexer_rules, &parser_rules) {
            Some(_errno) => prompt = "! ",
            None => prompt = "> ",
        }
    }
    None
}

fn argonaut_main() -> Option<i32> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    
    // Set variables
    let mut grammar_file = String::new();
    let mut source_file: Option<String> = None;
    let mut validate = false;

    let description = "
        Program for testing and validating HECK grammars.
    ";
    
    // Declare what arguments are expected and how to parse them
    match parse("heck", &args, vec![
        ArgDef::positional("grammar", &mut grammar_file)
            .help("The HECK grammar file to load.")
        
        , ArgDef::setting("source_file", &mut source_file)
            .short("i")
            .help("An optional source file to try to parse")

        , ArgDef::flag("validate", &mut validate)
            .short("v")
            .help("Validates the grammar, without starting the REPL")
        
        , help_arg(description).short("h")
        , version_arg()
    ]) {
        Ok(_optional_error_code) => {},
        Err(ParseError::Interrupted(_)) => {
            return None;
        },
        Err(_) => {
            return Some(1);
        }
    };
    
    // Use the parsed arguments after a succesful parse
    let p = Path::new(&grammar_file);
    if ! grammar_file.ends_with(".heck") {
        println!("The grammar file should end with '.heck'! ('{}')", grammar_file);
        return Some(1);
    }
    let mut file = match File::open(&p) {
        Ok(file) => file,
        Err(err) => {
            println!("Could not open grammar file: {}", err.description());
            return Some(1);
        }
    };
    let mut grammar = String::new();
    if let Err(err) = file.read_to_string(&mut grammar) {
        println!("Could not read grammar file: {}", err.description());
        return Some(1);
    }
    if let Some(source_file) = source_file {
        let mut sf = match File::open(&source_file) {
            Ok(sf) => sf,
            Err(err) => {
                println!("Could not open source file: {}", err.description());
                return Some(1);
            }
        };
        let mut source = String::new();
        if let Err(err) = sf.read_to_string(&mut source) {
            println!("Could not read source file: {}", err.description());
            return Some(1);
        }
        let raw_rules = match parse_raw_rules(&grammar) {
            Ok(rules) => rules,
            Err(err) => {
                println!("Could not parse grammar: {}", err);
                return Some(1);
            }
        };
        let lexer_rules = find_lexer_rules(&raw_rules);
        let parser_rules = find_parser_rules(&raw_rules);
        try_parse(&source, &lexer_rules, &parser_rules)
    } else {
        run_prompt(&grammar);
        None
    }
}

