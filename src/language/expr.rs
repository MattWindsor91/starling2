//! Expressions in the high-level Starling language.

use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

use crate::language::expr::map::HasMeta;
pub use bop::Bop;
pub use constant::Constant;
use map::HasVars;
pub use uop::Uop;

use super::{tagged, var::Variable};

pub mod bop;
pub mod constant;
pub mod egg;
pub mod map;
pub mod uop;

/// The body of an expression, parameterised over tags and variables.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr<M, V> {
    /// Literal expression.
    Literal(tagged::Tagged<M, Constant>),
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
    pub fn bop(lhs: impl Into<Box<Self>>, op: impl Into<Bop>, rhs: impl Into<Box<Self>>) -> Self {
        Self::Bop {
            lhs: lhs.into(),
            op: op.into(),
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
        Self::Literal(tagged::Tagged::with_default(Constant::Bool(value)))
    }

    /// Convenience constructor for an integer literal with no tag.
    #[must_use]
    pub fn i64(value: i64) -> Self {
        Self::Literal(tagged::Tagged::with_default(Constant::Int(
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

impl<M, V: Variable> HasVars<V> for Expr<M, V> {
    type Output<U> = Expr<M, U>;

    fn try_map_var<U, E>(self, f: impl FnMut(V) -> Result<U, E>) -> Result<Self::Output<U>, E> {
        VarMapper {
            f,
            v: PhantomData::default(),
            e: PhantomData::default(),
        }
        .try_map(self)
    }
}

/// Maps a closure over variables inside expressions, while retaining ownership over the closure.
struct VarMapper<V, U, E, F: FnMut(V) -> Result<U, E>> {
    f: F,
    v: PhantomData<V>,
    e: PhantomData<E>,
}

impl<V: Variable, U, E, F: FnMut(V) -> Result<U, E>> VarMapper<V, U, E, F> {
    fn try_map<M>(&mut self, e: Expr<M, V>) -> Result<Expr<M, U>, E> {
        match e {
            Expr::Literal(l) => Ok(Expr::Literal(l)),
            Expr::Var(v) => v.try_map_var(&mut self.f).map(Expr::Var),
            Expr::Bop { op, lhs, rhs } => {
                let lhs: Expr<M, U> = self.try_map(*lhs)?;
                let rhs: Expr<M, U> = self.try_map(*rhs)?;
                Ok(Expr::bop(lhs, op, rhs))
            }
            Expr::Uop { op, expr } => {
                let expr: Expr<M, U> = self.try_map(*expr)?;
                Ok(Expr::uop(op, expr))
            }
        }
    }
}

impl<M, V> HasMeta<M> for Expr<M, V> {
    type Output<N> = Expr<N, V>;

    fn try_map_meta<N, E>(self, f: impl FnMut(M) -> Result<N, E>) -> Result<Self::Output<N>, E> {
        MetaMapper {
            f,
            v: PhantomData::default(),
            e: PhantomData::default(),
        }
        .try_map(self)
    }
}

/// Maps a closure over metadata inside expressions, while retaining ownership over the closure.
struct MetaMapper<M, N, E, F: FnMut(M) -> Result<N, E>> {
    f: F,
    v: PhantomData<M>,
    e: PhantomData<E>,
}

impl<M, N, E, F: FnMut(M) -> Result<N, E>> MetaMapper<M, N, E, F> {
    fn try_map<V>(&mut self, e: Expr<M, V>) -> Result<Expr<N, V>, E> {
        match e {
            Expr::Literal(l) => l.try_map_direct_meta(&mut self.f).map(Expr::Literal),
            Expr::Var(v) => v.try_map_direct_meta(&mut self.f).map(Expr::Var),
            Expr::Bop { op, lhs, rhs } => {
                let lhs = self.try_map(*lhs)?;
                let rhs = self.try_map(*rhs)?;
                Ok(Expr::bop(lhs, op, rhs))
            }
            Expr::Uop { op, expr } => {
                let expr = self.try_map(*expr)?;
                Ok(Expr::uop(op, expr))
            }
        }
    }
}

/// A tagged expression.
pub type Tagged<M, V> = tagged::Tagged<M, Expr<M, V>>;
