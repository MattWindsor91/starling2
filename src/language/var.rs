//! Variable-related types common to multiple representations.

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
