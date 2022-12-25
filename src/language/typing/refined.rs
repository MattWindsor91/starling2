//! Refinement annotations.

use std::fmt::{Display, Formatter};

/// Base structure of a refined type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Refined<T, R> {
    /// The type being refined.
    element: Box<T>,
    /// The refinement variable.
    var: String,
    /// The refinement predicate (over `var`).
    refinement: R,
}

impl<T: Display, R: Display> Display for Refined<T, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self {
            element,
            var,
            refinement,
        } = self;
        write!(f, "{{{var}: {element} | {refinement}}}")
    }
}
