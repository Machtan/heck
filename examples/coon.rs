extern crate heck;
extern crate argonaut;

use heck::{Match, lex_and_parse_with_grammar};
use std::collections::HashMap;
use argonaut::{ArgDef, parse, ParseError, help_arg, version_arg};
use std::process;
use std::env;
use std::fs::File;
use std::error::Error;
use std::io::Read;

const GRAMMAR: &str = r##"

"##;


fn main() {
    // Properly set exit codes after the program has cleaned up.
    if let Some(exit_code) = argonaut_main() {
        process::exit(exit_code);
    }
}

fn argonaut_main() -> Option<i32> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    
    // Set variables
    let mut file = String::new();

    let description = "
        Parses and runs a Coon file.
    ";
    
    // Declare what arguments are expected and how to parse them
    match parse("coon", &args, vec![
        ArgDef::positional("file", &mut file)
            .help("The file to run.")
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
    
    let mut f = match File::open(&file) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Could not open passed file: {}", err.description());
            return Some(1);
        }
    };
    let mut text = String::new();
    if let Err(err) = f.read_to_string(&mut text) {
        eprintln!("Could not read file: {}", err.description());
        return Some(1);
    }
    println!("Running Coon parser...");
    let mtc = match lex_and_parse_with_grammar(&text, GRAMMAR, "document") {
        Ok(mtc) => mtc,
        Err(err) => {
            eprintln!("Parsing failed:\n{}", err);
            return Some(2);
        }
    };
    println!("Match: {:#?}", mtc);
    
    // Return no error code
    None
}



