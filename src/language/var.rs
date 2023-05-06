//! Variable-related types common to multiple representations.

use egg::Symbol;

/// A variable scope.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum Scope {
    /// Shared scope.
    #[default]
    Shared,
    /// Thread-local scope.
    Thread,
}

/// A variable reference.
pub trait Variable {
    /// Maps a variable to an `egg` symbol, possibly performing some mangling.
    fn to_symbol(&self) -> Symbol;

    /// As `to_symbol`, but takes ownership of the entire variable reference.
    fn into_symbol(self) -> Symbol;
}
