//! View declarations.
//!
//! View declarations tell Starling what the expected shape of view atoms are.

use super::super::{super::tagged::Tagged, call};

/// A declaration for one or more view atoms.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Decl<'inp, M, V> {
    /// The contents of the declaration.
    pub contents: Vec<Tagged<M, Prototype<'inp, M, V>>>,
}

/// The default declaration is an empty one.
///
/// We can't derive this because that would introduce unnecessary constraints on `M` and `V`.
impl<'inp, M, V> Default for Decl<'inp, M, V> {
    fn default() -> Self {
        Self { contents: vec![] }
    }
}

/// A view prototype.
pub type Prototype<'inp, M, V> = call::Prototype<'inp, M, V>;
