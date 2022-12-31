//! Constraint bodies.

use super::{super::tagged::Tagged, expr, view};
use crate::language::ast::view::pattern;

/// A constraint declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Decl<'inp, M, V> {
    /// List of views that are bound by this constraint.
    pub views: Vec<Tagged<M, pattern::Pattern<'inp, M, V>>>,
    /// Body of the constraint.
    pub body: Tagged<M, Constraint<'inp, M, V>>,
}

impl<'inp, M: Default, V> Default for Decl<'inp, M, V> {
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
pub enum Constraint<'inp, M, V> {
    /// An entailment constraint body.
    Entails(Entailment<'inp, M, V>),
}

/// A constraint body that maps a view pattern to a Boolean expression.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Entailment<'inp, M, V> {
    /// The expression entailed by the constraint.
    pub entails: Tagged<M, expr::Expr<'inp, M, V>>,
}

/// The default entailment constraint is 'true'.
impl<'inp, M: Default, V> Default for Entailment<'inp, M, V> {
    fn default() -> Self {
        Self {
            entails: Tagged::with_default(expr::Expr::bool(true)),
        }
    }
}
