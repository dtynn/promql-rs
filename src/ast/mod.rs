use std::convert::From;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use lrpar::Lexeme;

pub(crate) mod util;

pub(crate) type MaybeLexme = std::result::Result<Lexeme<u32>, Lexeme<u32>>;

#[derive(Debug, Clone)]
pub enum Expr {
    NumberLiteral(Number),
    ParenExpr(ParenExpr),
    BinaryExpr(BinaryExpr),
    UnaryExpr(UnaryExpr),
}

macro_rules! impl_from_for_expr {
    ($t:ident) => {
        impl From<$t> for Expr {
            fn from(val: $t) -> Expr {
                Expr::$t(val)
            }
        }
    };

    ($name:ident, $t:ty) => {
        impl From<$t> for Expr {
            fn from(val: $t) -> Expr {
                Expr::$name(val)
            }
        }
    };
}

impl_from_for_expr!(NumberLiteral, Number);
impl_from_for_expr!(ParenExpr);
impl_from_for_expr!(BinaryExpr);
impl_from_for_expr!(UnaryExpr);

#[derive(Debug, Copy, Clone)]
pub enum Operator {
    ADD,
    SUB,
    MUL,
    DIV,
}

impl FromStr for Operator {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        match input {
            "+" => Ok(Self::ADD),
            "-" => Ok(Self::SUB),
            "*" => Ok(Self::MUL),
            "/" => Ok(Self::DIV),
            _ => Err(anyhow!("unknown op {}", input)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub op: Operator,
    pub lh: Box<Expr>,
    pub rh: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(lh: Expr, op: Operator, rh: Expr) -> Self {
        BinaryExpr {
            op,
            lh: Box::new(lh),
            rh: Box::new(rh),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParenExpr(pub Box<Expr>);

impl ParenExpr {
    pub fn new(expr: Expr) -> Self {
        ParenExpr(Box::new(expr))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOp {
    ADD,
    SUB,
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(op: UnaryOp, expr: Expr) -> UnaryExpr {
        UnaryExpr {
            op,
            expr: Box::new(expr),
        }
    }
}

impl FromStr for UnaryOp {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        match input {
            "+" => Ok(Self::ADD),
            "-" => Ok(Self::SUB),
            _ => Err(anyhow!("unknown op {}", input)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Number(pub f64);

impl From<f64> for Number {
    fn from(val: f64) -> Self {
        Number(val)
    }
}

impl From<i64> for Number {
    fn from(val: i64) -> Self {
        Number(val as f64)
    }
}
