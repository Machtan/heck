extern crate heck;

use heck::{Match, lex_and_parse_with_grammar};
use std::collections::HashMap;

const TOML_GRAMMAR: &str = r##"
_SPACE:        " "
_TAB:          "\t"
_COMMENT:    r#"#.*?"#
TRUE:         "true"
FALSE:        "false"
NEWLINE:      "\r\n" | "\n"
STRING:     r#""(?:\.|[^"])*""#
IDENT:      r#"[a-zA-Z_][a-zA-Z_0-9\-]*"#
FLOAT:      r#"(?:\+|-)[0-9](?:_?[0-9])*\.(?:[0-9](?:_?[0-9])*)?"#
INT:        r#"(?:\+|-)[0-9](_?[0-9])*"#

key:    $(KEY | STRING)
endl:   NEWLINE
end:    (EOF | NEWLINE)
table_scope:  "[" $$key ( "." $$key )* "]"
aot_scope: "[[" $$key ( "." $$key )* "]]"
array:  "[" endl* "]"! $$expr "]"! ("," endl* "]"! $$expr endl* "]"!)%
expr:   $(INT | FLOAT | STRING | array | inline_table | TRUE | FALSE)
entry:  $key "=" $expr
inline_table: "{" endl* "}"! $$entry "}"! ("," endl* "}"! $$entry endl* "}"!)%


document:   (($$entry | $$aot_scope | $$table_scope)? end)+

"##;

const TOML_FILE: &str = include_str!("../Cargo.toml");

type TomlTable = HashMap<String, TomlValue>;

#[derive(Debug, Clone)]
pub enum TomlValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Array(Vec<TomlValue>),
    Table(TomlTable),
}

type TomlResult<T> = Result<T, String>;

fn with_scope<'a, F: FnOnce(&'a mut TomlTable), I: Iterator<Item=S>, S: ToString> (table: &'a mut TomlTable, mut path: I, f: F) {
    match path.next() {
        None => {
            f(table)
        }
        Some(key) => {
            let next = table.entry(key.to_string()).or_insert_with(|| TomlValue::Table(TomlTable::new()));
            match next {
                &mut TomlValue::Table(ref mut table) => {
                    with_scope(table, path, f);
                }
                &mut TomlValue::Array(ref mut values) => {
                    if let Some(val) = values.last_mut() {
                        if let &mut TomlValue::Table(ref mut table) = val {
                            with_scope(table, path, f);
                        } else {
                            panic!("Scope path part is not table or array of tables");
                        }
                    } else {
                        panic!("No values in array");
                    }
                }
                _ => {
                    panic!("Scope path part is not table or array of tables :c");
                }
            }
        }
    }
}

// TODO: if I want it to parse 'real' toml :p
fn clean_string(s: &str) -> String {
    s.to_string()
}

fn reduce_key(m: &Match, source: &str) -> String {
    let m = m.single(0).unwrap();
    match m.rule.as_str() {
        "STRING" => clean_string(m.token().unwrap().slice(source)),
        "KEY" => m.token().unwrap().slice(source).to_string(),
        _ => unreachable!(),
    }
}

fn reduce_inline_table(m: &Match, source: &str) -> TomlResult<TomlValue> {
    let mut table = TomlTable::new();
    for res in m.multiple(0).unwrap().into_iter().map(|m| reduce_entry(m, source)) {
        let (k, v) = res?;
        table.insert(k, v);
        // TODO: ensure keys only added once.
    }
    Ok(TomlValue::Table(table))
}

fn reduce_array(m: &Match, source: &str) -> TomlResult<TomlValue> {
    let mut arr = Vec::new();
    for res in m.multiple(0).unwrap().into_iter().map(|m| reduce_expr(m, source)) {
        arr.push(res?);
    }
    Ok(TomlValue::Array(arr))
}

fn reduce_expr(m: &Match, source: &str) -> TomlResult<TomlValue> {
    let m = m.single(0).unwrap();
    Ok(match m.rule.as_str() {
        "INT" => {
            TomlValue::Int(m.token().unwrap().slice(source).parse().expect("Invalid int :c"))
        }
        "FLOAT" => {
            TomlValue::Float(m.token().unwrap().slice(source).parse().expect("Invalid float :c"))
        }
        "STRING" => {
            TomlValue::Str(clean_string(m.token().unwrap().slice(source)))
        }
        "array" => reduce_array(m, source)?,
        "inline_table" => reduce_inline_table(m, source)?,
        "TRUE" => TomlValue::Bool(true),
        "FALSE" => TomlValue::Bool(false),
        _ => unreachable!(),
    })
}

fn reduce_entry(m: &Match, source: &str) -> TomlResult<(String, TomlValue)> {
    let key = reduce_key(m.single(0).unwrap(), source);
    let value = reduce_expr(m.single(1).unwrap(), source)?;
    Ok((key, value))
}

fn reduce_scope(m: &Match, source: &str) -> Vec<String> {
    m.multiple(0).unwrap().into_iter().map(|m| reduce_key(m, source)).collect()
}

fn reduce_document(m: &Match, source: &str) -> TomlResult<TomlTable> {
    let mut table = HashMap::new();
    let mut scope = Vec::new();
    for tlitem in m.multiple(0).unwrap() {
        match tlitem.rule.as_str() {
            "table_scope" => {
                scope = reduce_scope(tlitem, source);
            }
            "aot_scope" => {
                scope = reduce_scope(tlitem, source);
                let (last, pre) = scope.split_last().unwrap();
                with_scope(&mut table, pre.iter(), |table| {
                    let val = table.entry(last.to_string()).or_insert_with(
                        || TomlValue::Array(Vec::new())
                    );
                    if let &mut TomlValue::Array(ref mut values) = val {
                        values.push(TomlValue::Table(HashMap::new()));
                    } else {
                        panic!("AOT scope is not an array!");
                    }
                });
            }
            "entry" => {
                let (key, val) = reduce_entry(tlitem, source)?;
                with_scope(&mut table, scope.iter(), |table| {
                    table.insert(key, val);
                });
            }
            _ => unreachable!(),
        }
    }
    Ok(table)
}

fn main() {
    let mtc = lex_and_parse_with_grammar(TOML_FILE, TOML_GRAMMAR, "document")
        .expect("Could not parse TOML document");
    let document = reduce_document(&mtc, TOML_FILE).expect("Invalid document");
    println!("Document: {:#?}", document);
}
