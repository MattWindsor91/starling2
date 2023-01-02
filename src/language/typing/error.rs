//! Errors relating to the type system.

/// A PVC typing error.
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error<M, V> {
    /// A typing mismatch.
    #[error("expected type {want}, got type {got}")]
    Mismatch {
        want: super::Type<M, V>,
        got: super::Type<M, V>,
    },
}

/// Shorthand for results over [Error].
pub type Result<T, M, V> = std::result::Result<T, Error<M, V>>;
