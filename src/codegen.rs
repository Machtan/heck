
use parser::ParserRules;

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
    for (_, ref rule) in parser_rules {
        let mut signature = String::new();
        signature.push_str(&format!(
            "fn reduce_{}(m: &Match, source: &str) -> T {{", rule.name
        ));
        for (i, &cap) in rule.captures.iter().enumerate() {
            match cap {
                Single => {
                    signature.push_str(&format!(
                        "\n    let arg_{} = m.single({}).unwrap();", i, i
                    ));
                }
                Optional => {
                    signature.push_str(&format!(
                        "\n    let arg_{} = m.optional({}).unwrap();", i, i
                    ));
                }
                Multiple => {
                    signature.push_str(&format!(
                        "\n    for arg_{} in m.multiple({}).unwrap() {{\n        \n    }}", i, i
                    ));
                }
            }
        }
        signature.push_str("\n}");
        signatures.push(signature);
    }
    signatures
}