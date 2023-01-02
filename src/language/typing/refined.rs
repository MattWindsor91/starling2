//! Refinement annotations.

use std::fmt::{Display, Formatter};

/// Base structure of a refined type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Refined<M, V> {
    /// The type being refined.
    element: Box<super::Type<M, V>>,
    /// The refinement variable.
    var: String,
    /// The refinement predicate (over `var`).
    refinement: super::super::expr::Expr<M, V>,
}

impl<M, V: Display> Display for Refined<M, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self {
            element,
            var,
            refinement,
        } = self;
        write!(f, "{{{var}: {element} | {refinement}}}")
    }
}
