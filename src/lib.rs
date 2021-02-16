#![deny(missing_docs)]
//! promql-rs is a rust implementation of Prometheus Query Language
//!

#[macro_use]
extern crate pest_derive;

use anyhow::{anyhow, Result};
use pest::Parser;

mod convert;
mod parser;

type Pair<'i> = pest::iterators::Pair<'i, parser::Rule>;

/// Parses an input str of a prometheus query
pub fn parse(raw: &str) -> Result<()> {
    let input: Pair = parser::PromQLParser::parse(parser::Rule::input, raw)?
        .next()
        .ok_or(anyhow!("no input body found"))?;

    for sub in input.into_inner() {
        match sub.as_rule() {
            _ => unreachable!(),
        }
    }

    unimplemented!();
}
