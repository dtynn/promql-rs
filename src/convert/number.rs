use anyhow::{anyhow, Result};

use super::from_str;
use crate::{parser::Rule, Pair};

fn number(p: &Pair) -> Result<f64> {
    from_str(p)
}

fn uint(p: &Pair) -> Result<u64> {
    from_str(p)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn number_test() {
        let cases = vec![
            ("0", 0.0),
            ("1", 1f64),
            ("1.0", 1f64),
            ("10.1", 10.1),
            ("10e5", 10e5f64),
            ("10e-5", 10e-5f64),
            ("10.1e-5", 10.1e-5f64),
        ];

        for c in cases {
            let p = parser::PromQLParser::parse(parser::Rule::number, c.0)
                .unwrap()
                .next()
                .unwrap();

            assert_eq!(p.as_rule(), parser::Rule::number);
            assert_eq!(number(&p).unwrap(), c.1, "input: {}", c.0);
        }
    }

    #[test]
    fn uint_test() {
        let cases = vec![
            ("0", 0u64),
            ("1", 1u64),
            ("10", 10u64),
            ("101", 101u64),
            ("100000", 100_000u64),
            ("1000000000", 1000_000_000u64),
        ];

        for c in cases {
            let p = parser::PromQLParser::parse(parser::Rule::uint, c.0)
                .unwrap()
                .next()
                .unwrap();

            assert_eq!(p.as_rule(), parser::Rule::uint);
            assert_eq!(uint(&p).unwrap(), c.1, "input: {}", c.0);
        }
    }
}
