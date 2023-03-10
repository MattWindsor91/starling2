//! View assertions.

use super::{
    super::{
        super::{expr, tagged::Tagged},
        call, ite,
    },
    Iterated,
};

/// A view assertion.
///
/// View assertions form a rich expression language, so as to make it easier to annotate proofs with
/// pre- and post-conditions.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Assertion<'inp, M, V> {
    /// A view atom.
    Atom(Tagged<M, Atom<'inp, M, V>>),
    /// An empty view.
    Emp,
    /// A guarded view assertion.
    Guarded(Guarded<'inp, M, V>),
    /// An if-then-else construct.
    ///
    /// This is semantically equivalent to the join of `true_view` guarded by `cond`, and
    /// `false_view` guarded by the negation of `cond`.  Normalising
    Ite(Ite<'inp, M, V>),
    /// An iteration.
    ///
    /// Entire atoms can be iterated, with the result being flattened into individual guarded
    /// iterated views later on.
    Iterated(Iterated<M, V, Box<Assertion<'inp, M, V>>>),
    /// A join of two assertions.
    Join(Box<Assertion<'inp, M, V>>, Box<Assertion<'inp, M, V>>),
    /// A local assertion.
    ///
    /// This is a Boolean expression over local variables, which is mixed into non-local views.
    Local(expr::Tagged<M, V>),
}

/// A guarded view.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Guarded<'inp, M, V> {
    /// The guard.
    pub guard: expr::Expr<M, V>,
    /// The view being guarded.
    pub view: Box<Assertion<'inp, M, V>>,
}

/// An if-then-else.
pub type Ite<'inp, M, V> = ite::Ite<M, V, Box<Assertion<'inp, M, V>>>;

/// A view assertion atom.
///
/// Assertion atoms in the AST do not have iterators.
pub type Atom<'inp, M, V> = call::Generic<'inp, M, expr::Expr<M, V>>;
