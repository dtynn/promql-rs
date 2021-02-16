use anyhow::{anyhow, Result};

use super::from_str;
use crate::{parser::Rule, Pair};

fn string_literal(pair: Pair) -> Result<String> {
    pair.into_inner()
        .into_iter()
        .map(|p| match p.as_rule() {
            Rule::char_raw => from_str::<char>(&p),
            Rule::char_byte_inner | Rule::char_unicode_inner => char_from_hex(&p),
            Rule::char_predefined_inner => char_predefined(&p),
            other => Err(anyhow!(
                "unexpected rule type {:?} in string_literal",
                other
            )),
        })
        .collect()
}

fn char_predefined(p: &Pair) -> Result<char> {
    match p.as_str() {
        "n" => Ok('\n'),
        "r" => Ok('\r'),
        "t" => Ok('\t'),
        "\\" => Ok('\\'),
        "0" => Ok('\0'),
        "\"" => Ok('\"'),
        "'" => Ok('\''),
        other => Err(anyhow!("unexpected predifined char {}", other)),
    }
}

fn char_from_hex(p: &Pair) -> Result<char> {
    let s = p.as_str();
    let n = u32::from_str_radix(s, 16)?;
    std::char::from_u32(n).ok_or(anyhow!("unable to convert from hex {} to char", s))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn string_literal_test() {
        let pair = parser::PromQLParser::parse(
            parser::Rule::string_literal,
            r#""a\n\r\t\\\0\"\'b\x0Fc\u{a}d\u{00e9}e""#,
        )
        .unwrap()
        .next()
        .unwrap();

        assert_eq!(
            string_literal(pair).unwrap(),
            String::from("a\n\r\t\\\0\"\'b\x0fc\u{a}d\u{00e9}e")
        )
    }
}
