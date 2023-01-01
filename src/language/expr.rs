//! Expressions in the high-level Starling language.

use std::fmt::{Display, Formatter};

pub use bop::Bop;
pub use literal::Literal;
pub use uop::Uop;

use super::tagged;

pub mod bop;
mod egg;
pub mod literal;
pub mod uop;

/// The body of an expression, parameterised over tags and variables.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr<M, V> {
    /// Literal expression.
    Literal(tagged::Tagged<M, Literal>),
    /// Variable reference.
    Var(tagged::Tagged<M, V>),
    /// Binary (infix) operation.
    Bop {
        op: Bop,
        lhs: Box<Expr<M, V>>,
        rhs: Box<Expr<M, V>>,
    },
    /// Unary (prefix or postfix) operation.
    Uop { op: Uop, expr: Box<Expr<M, V>> },
}

impl<M, V> Expr<M, V> {
    /// Convenience constructor for a binary operation.
    #[must_use]
    pub fn bop(lhs: impl Into<Box<Self>>, op: Bop, rhs: impl Into<Box<Self>>) -> Self {
        Self::Bop {
            lhs: lhs.into(),
            op,
            rhs: rhs.into(),
        }
    }

    /// Convenience constructor for a dereference operation.
    #[must_use]
    pub fn deref(expr: impl Into<Box<Self>>) -> Self {
        Self::uop(Uop::Deref, expr)
    }

    /// Convenience constructor for a unary operation.
    #[must_use]
    pub fn uop(op: Uop, expr: impl Into<Box<Self>>) -> Self {
        Self::Uop {
            op,
            expr: expr.into(),
        }
    }
}

impl<M: Default, V> Expr<M, V> {
    /// Convenience constructor for a Boolean literal with no tag.
    #[must_use]
    pub fn bool(value: bool) -> Self {
        Self::Literal(tagged::Tagged::with_default(Literal::Bool(value)))
    }

    /// Convenience constructor for an integer literal with no tag.
    #[must_use]
    pub fn i64(value: i64) -> Self {
        Self::Literal(tagged::Tagged::with_default(Literal::Int(
            num_bigint::BigInt::from(value),
        )))
    }
}

impl<M, V: Display> Display for Expr<M, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(l) => l.fmt(f),
            Self::Var(v) => v.fmt(f),
            // TODO: precedence
            Self::Bop { op, lhs, rhs } => write!(f, "({lhs}) {op} ({rhs})"),
            Self::Uop { op, expr } => match op.fixity() {
                uop::Fixity::Prefix => write!(f, "{op}({expr})"),
                uop::Fixity::Postfix => write!(f, "({expr}){op}"),
            },
        }
    }
}

/// A tagged expression.
pub type Tagged<M, V> = tagged::Tagged<M, Expr<M, V>>;
