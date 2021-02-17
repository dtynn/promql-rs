use anyhow::{anyhow, Result};

use crate::{parser::Rule, Pair};

fn metric_identifier(pair: Pair) -> Result<Vec<String>> {
    pair.into_inner()
        .into_iter()
        .map(|p| match p.as_rule() {
            Rule::identifier => Ok(p.as_str().to_owned()),
            other => Err(anyhow!(
                "unexpected rule type {:?} inside metric_identifier",
                other
            )),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn metric_identifier_test() {
        let cases = vec![(
            "test:a:b",
            vec![String::from("test"), String::from("a"), String::from("b")],
        )];

        for c in cases {
            let p = parser::PromQLParser::parse(parser::Rule::metric_identifier, c.0)
                .unwrap()
                .next()
                .unwrap();

            assert_eq!(p.as_rule(), parser::Rule::metric_identifier);
            assert_eq!(metric_identifier(p).unwrap(), c.1, "input: {}", c.0);
        }
    }

    #[test]
    fn metric_test() {
        let cases = vec![(r#"test:a:b{a="b", b="c", c="d"}"#, ())];

        for c in cases {
            let p = parser::PromQLParser::parse(parser::Rule::metric, c.0)
                .unwrap()
                .next()
                .unwrap();

            assert_eq!(p.as_rule(), parser::Rule::metric);
        }
    }
}
