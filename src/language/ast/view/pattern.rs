//! View patterns and their components.

use super::super::super::{expr::Expr, tagged::Tagged};

/// A view pattern (list of atoms).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pattern<M, V> {
    pub contents: Vec<Tagged<M, Atom<M, V>>>,
}

/// The default view pattern is `emp`.
impl<M, V> Default for Pattern<M, V> {
    fn default() -> Self {
        Self { contents: vec![] }
    }
}

/// A view atom pattern.
///
/// Atom patterns are iterated, but not guarded.  We nest the iterator into the pattern because,
/// unlike `Iterated`, the iterator can be a pattern argument.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Atom<M, V> {
    /// The name of the atom.
    pub name: Tagged<M, V>,
    /// The arguments of the atom.
    pub args: Vec<Tagged<M, Argument<M, V>>>,
    /// The iterator.
    pub iterator: Tagged<M, Argument<M, V>>,
}

/// The default atom pattern is the default name with no arguments and iterator `1`.
impl<M: Default, V: Default> Default for Atom<M, V> {
    fn default() -> Self {
        Self {
            name: Tagged::default(),
            args: vec![],
            iterator: Tagged::with_default(Argument::Expr(Expr::i64(1))),
        }
    }
}

/// A view argument pattern.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Argument<M, V> {
    /// A `_` argument.
    Wildcard,
    /// An expression used as a pattern.
    Expr(Expr<M, V>),
}
