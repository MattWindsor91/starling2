//! Programs, procedures, and declarations.

use super::{super::tagged::Tagged, call::Prototype, constraint, stm, var, view, Identifier};

/// A program.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Program<'inp, M, V> {
    /// The name of the program.
    pub name: Tagged<M, Identifier<'inp>>,
    /// The declarations within the program.
    pub decls: Vec<Tagged<M, Decl<'inp, M, V>>>,
}

/// A default program has no declarations and a blank name.
///
/// An empty name is not syntactically valid, but we assume that users of the default program will
/// replace it.
impl<'inp, M: Default, V> Default for Program<'inp, M, V> {
    fn default() -> Self {
        Self {
            name: Tagged::default(),
            decls: vec![],
        }
    }
}

/// A top-level declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Decl<'inp, M, V> {
    /// A constraint declaration.
    Constraint(constraint::Decl<M, V>),
    /// A procedure declaration.
    Procedure(Procedure<'inp, M, V>),
    /// A variable declaration.
    Var(var::Decl<'inp, M, V>),
    /// A view declaration.
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
