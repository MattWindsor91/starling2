//! Declarations in the Starling abstract syntax.

use super::{super::tagged::Tagged, call::Prototype, constraint, stm, view};

/// A top-level declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Decl<'inp, M, V> {
    Constraint(constraint::Decl<'inp, M, V>),
    Procedure(Procedure<'inp, M, V>),
    View(view::Decl<'inp, M, V>),
}

/// A procedure declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Procedure<'inp, M, V> {
    /// The procedure prototype.
    pub prototype: Tagged<M, Prototype<'inp, M, V>>,
    /// The body, as a list of tagged statement triples.
    pub body: stm::List<'inp, M, V>,
}

impl<'inp, M: Default, V> Default for Procedure<'inp, M, V> {
    fn default() -> Self {
        Self {
            prototype: Tagged::default(),
            body: vec![],
        }
    }
}
