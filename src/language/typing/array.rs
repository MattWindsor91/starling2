//! Array types in Starling.

use std::fmt::{Display, Formatter};

/// An array type.
///
/// Arrays in Starling are homogeneous, and have explicit low and high bound types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Array<M, V> {
    /// The base type of the array.
    base: Box<super::Type<M, V>>,
    /// The low bound of the array.
    low: usize,
    /// The high bound of the array.
    high: usize,
}

impl<M, V: Display> Display for Array<M, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{},{}]", self.base, self.low, self.high)
    }
}
