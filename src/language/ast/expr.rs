//! Expressions in the high-level Starling language.

pub mod bop;
pub mod literal;
pub mod uop;

use std::fmt::{Display, Formatter};

use super::super::tagged;
pub use bop::Bop;
pub use literal::Literal;
pub use uop::Uop;

/// The body of an expression, parameterised over tags and variables.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr<M, V> {
    /// Literal expression.
    Literal(Literal),
    /// Variable reference.
    Var(V),
    /// Binary (infix) operation.
    Bop {
        op: Bop,
        lhs: tagged::Box<M, Expr<M, V>>,
        rhs: tagged::Box<M, Expr<M, V>>,
    },
    /// Unary (prefix or postfix) operation.
    Uop {
        op: Uop,
        expr: tagged::Box<M, Expr<M, V>>,
    },
}

impl<T, V> Expr<T, V> {
    /// Convenience constructor for a Boolean literal.
    #[must_use]
    pub fn bool(value: bool) -> Self {
        Self::Literal(Literal::Bool(value))
    }
}

impl<T, V: Display> Display for Expr<T, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(l) => l.fmt(f),
            Self::Var(v) => v.fmt(f),
            // TODO: precedence
            Self::Bop { op, lhs, rhs } => write!(f, "({lhs}) {op} ({rhs})"),
            Self::Uop { op, expr } => write!(f, "{op}({expr})"),
        }
    }
}

/// An integer literal.
pub type IntLit = i64;
