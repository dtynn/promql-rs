use anyhow::{anyhow, Result};

use crate::Pair;

mod duration;

fn convert_from_str<O>(pair: &Pair) -> Result<O>
where
    O: std::str::FromStr,
    O::Err: std::fmt::Debug,
{
    pair.as_str().parse::<O>().map_err(|e| {
        anyhow!(
            "convert from {:?} to {}: {:?}",
            pair.as_rule(),
            std::any::type_name::<O>(),
            e
        )
    })
}
