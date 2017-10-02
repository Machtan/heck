
use parser::ParserRules;
use std::collections::HashSet;
use std::ops::Deref;

/*
Example signatures:

fn reduce_document(m: &Match, source: &str) -> TomlResult<TomlTable> {
    let mut table = HashMap::new();
    let mut scope = Vec::new();
    for tlitem in m.multiple(0).unwrap() {
        ...

fn reduce_entry(m: &Match, source: &str) -> TomlResult<(String, TomlValue)> {
    let key = reduce_key(m.single(0).unwrap(), source);
    let value = reduce_expr(m.single(1).unwrap(), source)?;
    Ok((key, value))
}

*/

/// Returns a list containing source code for functions to 'reduce' a match to
/// some kind of AST or value. Each parser rule in the given list will have
/// a corresponding signature with its name, in which the capture groups of
/// the match for this rule are properly unwrapped.
pub fn generate_reducer_signatures(parser_rules: &ParserRules) -> Vec<String> {
    use captures::CaptureType::*;
    let mut signatures = Vec::new();
    let rule_names = parser_rules.iter().map(|(ref name, _)| name.clone()).collect::<HashSet<_>>();
    for (_, ref rule) in parser_rules {
        let mut signature = String::new();
        signature.push_str(&format!(
            "fn reduce_{}(m: &Match, source: &str) -> T {{", rule.name
        ));
        for (i, &(ref name, cap)) in rule.captures.iter().enumerate() {
            let mut reducer = None;
            let capname = if let &Some(ref name) = name {
                if (name != rule.name.deref()) && rule_names.contains(name) {
                    reducer = Some(name.clone());
                }
                name.clone()
            } else {
                format!("cap_{}", i)
            };
            match cap {
                Single => {
                    signature.push_str(&if let Some(reducer) = reducer {
                        format!(
                            "\n    let {} = reduce_{}(m.single({}).unwrap(), source);", capname, reducer, i
                        )
                    } else {
                        format!(
                            "\n    let {} = m.single({}).unwrap();", capname, i
                        )
                    });
                }
                Optional => {
                    signature.push_str(&if let Some(reducer) = reducer {
                        format!(
                            "\n    let {} = reduce_{}(m.optional({}).unwrap(), source);", capname, reducer, i
                        )
                    } else {
                        format!(
                            "\n    let {} = m.optional({}).unwrap();", capname, i
                        )
                    });
                }
                Multiple => {
                    signature.push_str(&if let Some(reducer) = reducer {
                        format!(
                            "\n    for cap in m.multiple({}).unwrap() {{\n        let {} = reduce_{}(cap, source);\n    }}", i, capname, reducer
                        )
                    } else {
                        format!(
                            "\n    for {} in m.multiple({}).unwrap() {{\n        \n    }}", capname, i
                        )
                    });
                }
            }
        }
        signature.push_str("\n}");
        signatures.push(signature);
    }
    signatures
}