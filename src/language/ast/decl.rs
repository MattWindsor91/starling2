//! Declarations in the Starling abstract syntax.

use super::{call::Prototype, view, Statement};

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
    pub prototype: Prototype<'inp, M, V>,
    /// The body, as a list of statements.
    pub body: Vec<Statement>,
}

/// A declaration for one or more view atoms.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct View<'inp, M, V> {
    pub elements: Vec<view::Prototype<'inp, M, V>>,
}
