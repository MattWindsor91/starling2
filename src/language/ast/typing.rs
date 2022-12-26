//! Starling types at the high-level language level.
//!
//! This is a layer on the general `typing` module that injects Starling expressions into the
//! refinements position.  It also re-exports any typing types that do not have refinements.

use super::super::typing;

pub use super::super::typing::{Array, Prim};

/// Refined types, parametrised over the inner type, expression metadata, and variable encoding.
pub type Refined<'inp, I, M, V> = typing::Refined<I, super::Expr<'inp, M, V>>;

/// Types with expression refinements, parametrised over the expression metadata, and variable encoding.
pub type Type<'inp, M, V> = typing::Type<super::Expr<'inp, M, V>>;
