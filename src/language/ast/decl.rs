//! Declarations in the Starling abstract syntax.

use super::{super::tagged::Tagged, call::Prototype, view, StatementWithViews};

/// A top-level declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Decl<'inp, M, V> {
    Constraint(super::Constraint<'inp, M, V>),
    Procedure(Procedure<'inp, M, V>),
    View(View<'inp, M, V>),
}

/// A procedure declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Procedure<'inp, M, V> {
    /// The procedure prototype.
    pub prototype: Tagged<M, Prototype<'inp, M, V>>,
    /// The body, as a list of tagged statements.
    pub body: Vec<Tagged<M, StatementWithViews<'inp, M, V>>>,
}

impl<'inp, M: Default, V> Default for Procedure<'inp, M, V> {
    fn default() -> Self {
        Self {
            prototype: Tagged::default(),
            body: vec![],
        }
    }
}

/// A declaration for one or more view atoms.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct View<'inp, M, V> {
    pub elements: Vec<view::Prototype<'inp, M, V>>,
}
