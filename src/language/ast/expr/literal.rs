//! Literal expressions and their various impls.

use std::fmt::{Display, Formatter};

/// Literal expressions.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Literal<'inp> {
    /// Integer constant.
    Int(Int<'inp>),
    /// Boolean constant.
    Bool(bool),
}

impl<'inp> Display for Literal<'inp> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i) => i.fmt(f),
            Self::Bool(b) => b.fmt(f),
        }
    }
}

/// An integer literal.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Int<'inp> {
    /// An integer that parsed successfully as an integer literal.
    I64(i64),
    /// An integer that overflowed, and is therefore stored as its string.
    Big(&'inp str),
}

impl<'inp> Display for Int<'inp> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I64(i) => i.fmt(f),
            Self::Big(b) => b.fmt(f),
        }
    }
}
