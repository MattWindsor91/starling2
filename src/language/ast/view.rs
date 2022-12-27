//! Abstract syntax for views.

pub use assertion::Assertion;

use super::{
    super::{tagged::Tagged, Expr},
    call,
};

pub mod assertion;

//
// Patterns
//

/// A view pattern.
pub type Pattern<'inp, M, V> = View<'inp, M, PatternArgument<'inp, M, V>>;

/// A view atom pattern.
pub type PatternAtom<'inp, M, V> = call::Generic<'inp, M, PatternArgument<'inp, M, V>>;

/// A view argument pattern.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum PatternArgument<'inp, M, V> {
    /// A `_` argument.
    Wildcard,
    /// An expression used as a pattern.
    Expr(Expr<'inp, M, V>),
}

//
// Assertions
//
/// An if-then-else view assertion.

//
// Prototypes
//

/// A view prototype.
pub type Prototype<'inp, M, V> = call::Prototype<'inp, M, V>;

//
// General structure
//

/// A generic flat view structure.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct View<'inp, M, T> {
    /// The contents of the view.
    pub contents: Vec<Tagged<M, call::Generic<'inp, M, T>>>,
}

/// The default view is the empty one.
impl<'inp, M, T> Default for View<'inp, M, T> {
    fn default() -> Self {
        Self { contents: vec![] }
    }
}

/// Wraps an item (view or atom) with an iterator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Iterated<'inp, M, V, T> {
    /// The iterated item.
    pub item: T,
    /// The iterator.
    pub iterator: Tagged<M, Expr<'inp, M, V>>,
}
