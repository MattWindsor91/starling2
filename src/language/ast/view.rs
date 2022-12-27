//! Abstract syntax for views.

use super::{
    super::{tagged::Tagged, Expr},
    call::Generic,
};

// TODO(@MattWindsor91): iterated views

//
// Patterns
//

/// A view pattern.
pub type Pattern<'inp, M, V> = View<'inp, M, PatternArgument<'inp, M, V>>;

/// A view atom pattern.
pub type PatternAtom<'inp, M, V> = Atom<'inp, M, PatternArgument<'inp, M, V>>;

/// A view argument pattern.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PatternArgument<'inp, M, V> {
    Wildcard,
    Expr(Expr<'inp, M, V>),
}

//
// Assertions
//

/// A view assertion.
pub type Assertion<'inp, M, V> = View<'inp, M, Expr<'inp, M, V>>;

/// A view assertion atom.
pub type AssertionAtom<'inp, M, V> = Atom<'inp, M, Expr<'inp, M, V>>;

//
// Prototypes
//

/// A view prototype.
pub type Prototype<'inp, M, V> = super::call::Prototype<'inp, M, V>;

//
// General structure
//

/// A generic view structure.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct View<'inp, M, T> {
    /// The contents of the view.
    pub contents: Vec<Tagged<M, Atom<'inp, M, T>>>,
}

/// The default view is the empty one.
impl<'inp, M, T> Default for View<'inp, M, T> {
    fn default() -> Self {
        Self { contents: vec![] }
    }
}

/// A generic view atom.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Atom<'inp, M, T> {
    /// The main part of the atom.
    pub head: Generic<'inp, M, T>,
    pub iterator: Option<Tagged<M, Expr<'inp, M, T>>>,
}

/// The default atom has a default head and empty iterator.
impl<'inp, M: Default, T> Default for Atom<'inp, M, T> {
    fn default() -> Self {
        Self {
            head: Generic::default(),
            iterator: None,
        }
    }
}
