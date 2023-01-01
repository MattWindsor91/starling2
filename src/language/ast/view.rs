//! Abstract syntax for views.

pub use assertion::Assertion;
pub use decl::Decl;
pub use pattern::Pattern;

use super::super::{expr::Expr, tagged::Tagged};

pub mod assertion;
pub mod decl;
pub mod pattern;

/// Wraps a view component with an iterator.
///
/// This is used for view assertions and flattened atoms, but not for patterns.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Iterated<'inp, M, V, T> {
    /// The iterated item.
    pub item: T,
    /// The iterator.
    pub iterator: Tagged<M, Expr<'inp, M, V>>,
}

/// The default iterated item is the default item with iterator `1`.
impl<'inp, M: Default, V, T: Default> Default for Iterated<'inp, M, V, T> {
    fn default() -> Self {
        Self::once(T::default())
    }
}

impl<'inp, M: Default, V, T> Iterated<'inp, M, V, T> {
    /// Promotes a non-iterated item to an iterated item by iterating it once.
    ///
    /// The iterator expression is synthesised and therefore has no tag.
    #[must_use]
    pub fn once(item: T) -> Self {
        Self::times(item, 1)
    }

    /// Promotes a non-iterated item to an iterated item by iterating it the given number of times.
    ///
    /// The iterator expression is synthesised and therefore has no tag.
    #[must_use]
    pub fn times(item: T, amount: i64) -> Self {
        Self {
            item,
            iterator: Tagged::with_default(Expr::i64(amount)),
        }
    }
}
