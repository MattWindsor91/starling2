//! Expressions in the high-level Starling language.

use std::fmt::{Display, Formatter};

pub use bop::Bop;
pub use literal::Literal;
pub use uop::Uop;

use super::super::tagged::Tagged;

pub mod bop;
pub mod literal;
pub mod uop;

/// The body of an expression, parameterised over tags and variables.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr<'inp, M, V> {
    /// Literal expression.
    Literal(Tagged<M, Literal<'inp>>),
    /// Variable reference.
    Var(Tagged<M, V>),
    /// Binary (infix) operation.
    Bop {
        op: Bop,
        lhs: Box<Expr<'inp, M, V>>,
        rhs: Box<Expr<'inp, M, V>>,
    },
    /// Unary (prefix or postfix) operation.
    Uop {
        op: Uop,
        expr: Box<Expr<'inp, M, V>>,
    },
}

impl<'inp, M: Default, V> Expr<'inp, M, V> {
    /// Convenience constructor for a Boolean literal.
    #[must_use]
    pub fn bool(value: bool) -> Self {
        Self::Literal(Tagged::with_default(Literal::Bool(value)))
    }
}

impl<'inp, M, V: Display> Display for Expr<'inp, M, V> {
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
