//! View assertions.

use super::{
    super::super::{tagged::Tagged, Expr},
    Atom,
};

/// A view assertion.
///
/// View assertions form a rich expression language, so as to make it easier to annotate proofs with
/// pre- and post-conditions.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Assertion<'inp, M, V> {
    /// A view atom.
    Atom(Tagged<M, AssertionAtom<'inp, M, V>>),
    /// A guarded view assertion..
    Guarded(Guarded<'inp, M, V>),
    /// An if-then-else construct.
    ///
    /// This is semantically equivalent to the join of `true_view` guarded by `cond`, and
    /// `false_view` guarded by the negation of `cond`.  Normalising
    Ite(Ite<'inp, M, V>),
    /// A join of two assertions.
    Join(Box<Assertion<'inp, M, V>>, Box<Assertion<'inp, M, V>>),
}

/// A guarded view.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Guarded<'inp, M, V> {
    /// The guard.
    pub guard: Expr<'inp, M, V>,
    /// The view being guarded.
    pub view: Box<Assertion<'inp, M, V>>,
}

/// An if-then-else.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ite<'inp, M, V> {
    /// The view that holds if `cond` is true.
    pub true_view: Box<Assertion<'inp, M, V>>,
    /// The condition.
    pub cond: Tagged<M, Expr<'inp, M, V>>,
    /// The view that holds if `cond` is false.
    pub false_view: Box<Assertion<'inp, M, V>>,
}

/// A view assertion atom.
pub type AssertionAtom<'inp, M, V> = Atom<'inp, M, Expr<'inp, M, V>>;
