//! Primitive Starling types.

use std::fmt::{Display, Formatter};

/// Enumeration of all primitive types.
///
/// The list of primitive Starling types is subject to future expansion.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Prim {
    /// Arbitrary-width integer.
    ///
    /// Integer widths are considered a specific form of refinement predicate.
    Int,
    /// Booleans (true and false).
    Bool,
}

impl Display for Prim {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Int => "int",
            Self::Bool => "bool"
        })
    }
}