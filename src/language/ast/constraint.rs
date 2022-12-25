//! Constraint bodies.

use super::view;

/// A constraint declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Constraint<'inp, M, V> {
    pub view: view::Pattern<'inp, M, V>,
    pub body: Body<M, V>,
}

/// The body of a constraint.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Body<M, V> {
    /// An expression constraint body.
    Expr(Expr<M, V>),
}

/// A constraint body that maps a view pattern to a Boolean expression.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Expr<M, V> {
    /// The expression entailed by the constraint.
    pub entails: super::expr::Expr<M, V>,
}
