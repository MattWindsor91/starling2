//! Abstract syntax for views.

use super::{super::Expr, call::Call};

// TODO(@MattWindsor91): iterated views

//
// Patterns
//

/// A view pattern.
pub type Pattern<'inp, M, V> = View<'inp, M, ArgumentPattern<M, V>>;

/// A view atom pattern.
pub type PatternAtom<'inp, M, V> = Atom<'inp, M, ArgumentPattern<M, V>>;

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ArgumentPattern<M, V> {
    Wildcard,
    Expr(Expr<M, V>),
}

//
// Assertions
//

/// A view assertion.
pub type Assertion<'inp, M, V> = View<'inp, M, Expr<M, V>>;

/// A view assertion atom.
pub type AssertionAtom<'inp, M, V> = Atom<'inp, M, Expr<M, V>>;

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
    pub contents: Vec<Atom<'inp, M, T>>,
}

/// The default view is the empty one.
impl<'inp, M, T> Default for View<'inp, M, T> {
    fn default() -> Self {
        Self { contents: vec![] }
    }
}

/// A generic view atom.
pub type Atom<'inp, M, T> = Call<'inp, M, T>;
