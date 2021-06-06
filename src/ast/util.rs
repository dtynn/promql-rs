use std::str::FromStr;

use anyhow::{Error, Result};
use log::trace;
use lrpar::NonStreamingLexer;

use super::MaybeLexme;

pub fn span_str<'lexer, 'input: 'lexer>(
    lexer: &'lexer dyn NonStreamingLexer<'input, u32>,
    lexme: MaybeLexme,
) -> Result<&'input str> {
    let span = lexme?.span();
    let s = lexer.span_str(span);
    trace!("str {} from span [{}, {})", s, span.start(), span.end());
    Ok(s)
}

pub fn any_from_str<T>(input: &str) -> Result<T>
where
    T: FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    input.parse().map_err(Error::new)
}

pub fn i64_from_str_radix(input: &str, radix: u32) -> Result<i64> {
    i64::from_str_radix(&input[2..], radix).map_err(Error::new)
}
