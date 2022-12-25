//! Starling's type system.
//!
//! Starling has a refinement type system based on a handful of primitive types.

pub mod array;
pub mod prim;
pub mod refined;

use std::fmt::{Display, Formatter};

pub use array::Array;
pub use prim::Prim;
pub use refined::Refined;

/// Enumeration of all types in the Starling type system.
///
/// This enumeration predicates over `P`, the type of predicate expressions.
/// It is non-exhaustive, as the set of types in Starling is subject to future expansion.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Type<P> {
    /// An array type.
    Array(Array<Type<P>>),
    /// A refined type.
    Refined(Refined<Type<P>, P>),
    /// A primitive type.
    Prim(Prim),
}

impl<P: Display> Display for Type<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Array(a) => a.fmt(f),
            Self::Refined(r) => r.fmt(f),
            Self::Prim(p) => p.fmt(f),
        }
    }
}
