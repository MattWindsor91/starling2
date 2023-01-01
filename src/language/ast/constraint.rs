//! Constraint bodies.

use super::{
    super::{expr, tagged::Tagged},
    view::pattern,
};

/// A constraint declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Decl<M, V> {
    /// List of views that are bound by this constraint.
    pub views: Vec<Tagged<M, pattern::Pattern<M, V>>>,
    /// Body of the constraint.
    pub body: Tagged<M, Constraint<M, V>>,
}

impl<M: Default, V> Default for Decl<M, V> {
    fn default() -> Self {
        Self {
            views: vec![],
            body: Tagged::with_default(Constraint::Entails(Entailment::default())),
        }
    }
}

/// The body of a constraint.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Constraint<M, V> {
    /// An entailment constraint body.
    Entails(Entailment<M, V>),
}

/// A constraint body that maps a view pattern to a Boolean expression.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Entailment<M, V> {
    /// The expression entailed by the constraint.
    pub entails: Tagged<M, expr::Expr<M, V>>,
}

/// The default entailment constraint is 'true'.
impl<M: Default, V> Default for Entailment<M, V> {
    fn default() -> Self {
        Self {
            entails: Tagged::with_default(expr::Expr::bool(true)),
        }
    }
}
