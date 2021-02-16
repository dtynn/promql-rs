use std::time::Duration;

use anyhow::{anyhow, Result};

use super::from_str;
use crate::parser::*;
use crate::Pair;

fn duration_unit(pair: &Pair) -> Result<Duration> {
    match pair.as_str() {
        "ns" => Ok(Duration::from_nanos(1)),
        "us" => Ok(Duration::from_micros(1)),
        "ms" => Ok(Duration::from_millis(1)),
        "s" => Ok(Duration::from_secs(1)),
        "m" => Ok(Duration::from_secs(60)),
        "h" => Ok(Duration::from_secs(3600)),
        "d" => Ok(Duration::from_secs(86400)),
        "w" => Ok(Duration::from_secs(86400 * 7)),
        "y" => Ok(Duration::from_secs(86400 * 365)),
        other => Err(anyhow!("unknown duration unit {}", other)),
    }
}

fn duration_part(pair: Pair) -> Result<Duration> {
    let subs: Vec<Pair> = pair.into_inner().collect();
    let val = from_str::<u32>(&subs[0])?;
    let unit = duration_unit(&subs[1])?;
    return Ok(unit * val);
}

fn duration(pair: Pair) -> Result<Duration> {
    let mut res = Duration::from_secs(0);
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::duration_part => {
                res += duration_part(p)?;
            }
            Rule::digits => {
                res += from_str::<u64>(&p).map(|n| Duration::from_nanos(n))?;
            }
            other => return Err(anyhow!("unexpected rule type {:?}", other)),
        }
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn duration_test() {
        let cases = vec![
            ("1", Duration::from_nanos(1)),
            ("1ns", Duration::from_nanos(1)),
            ("5ns", Duration::from_nanos(5)),
            ("1us", Duration::from_micros(1)),
            ("5us", Duration::from_micros(5)),
            ("1ms", Duration::from_millis(1)),
            ("5ms", Duration::from_millis(5)),
            ("1m1ms", Duration::from_secs(60) + Duration::from_millis(1)),
            ("5m5ms", Duration::from_secs(300) + Duration::from_millis(5)),
            (
                "1h1ms",
                Duration::from_secs(3600) + Duration::from_millis(1),
            ),
            (
                "5h5ms",
                Duration::from_secs(3600 * 5) + Duration::from_millis(5),
            ),
            (
                "1d1h1m",
                Duration::from_secs(86400) + Duration::from_secs(3600) + Duration::from_secs(60),
            ),
            (
                "5d5h5m",
                Duration::from_secs(86400 * 5)
                    + Duration::from_secs(3600 * 5)
                    + Duration::from_secs(60 * 5),
            ),
        ];

        for c in cases {
            let pair = parser::PromQLParser::parse(parser::Rule::duration, c.0)
                .unwrap()
                .next()
                .unwrap();
            assert_eq!(pair.as_rule(), parser::Rule::duration);
            assert_eq!(duration(pair).unwrap(), c.1, "input: {}", c.0);
        }
    }
}
