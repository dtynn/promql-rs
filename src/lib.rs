// #![deny(missing_docs)]
//! promql-rs is a rust implementation of Prometheus Query Language
//!

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

pub mod ast;

lrlex_mod!("promql.l");
lrpar_mod!("promql.y");

pub use promql_l::*;
pub use promql_y::*;
