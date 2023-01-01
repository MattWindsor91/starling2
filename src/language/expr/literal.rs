//! Literal expressions and their various impls.

use std::fmt::{Display, Formatter};

/// Literal expressions.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Literal {
    /// Integer constant.
    Int(num_bigint::BigInt),
    /// Boolean constant.
    Bool(bool),
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i) => i.fmt(f),
            Self::Bool(b) => b.fmt(f),
        }
    }
}
