//! View patterns and their components.

use super::super::super::{expr::Expr, tagged::Tagged};

/// A view pattern (list of atoms).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pattern<'inp, M, V> {
    pub contents: Vec<Tagged<M, Atom<'inp, M, V>>>,
}

/// The default view pattern is `emp`.
impl<'inp, M, V> Default for Pattern<'inp, M, V> {
    fn default() -> Self {
        Self { contents: vec![] }
    }
}

/// A view atom pattern.
///
/// Atom patterns are iterated, but not guarded.  We nest the iterator into the pattern because,
/// unlike `Iterated`, the iterator can be a pattern argument.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Atom<'inp, M, V> {
    /// The name of the atom.
    pub name: Tagged<M, V>,
    /// The arguments of the atom.
    pub args: Vec<Tagged<M, Argument<'inp, M, V>>>,
    /// The iterator.
    pub iterator: Tagged<M, Argument<'inp, M, V>>,
}

/// The default atom pattern is the default name with no arguments and iterator `1`.
impl<'inp, M: Default, V: Default> Default for Atom<'inp, M, V> {
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
pub enum Argument<'inp, M, V> {
    /// A `_` argument.
    Wildcard,
    /// An expression used as a pattern.
    Expr(Expr<'inp, M, V>),
}
