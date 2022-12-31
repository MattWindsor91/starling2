//! Statements.

use super::{super::tagged::Tagged, call, expr, ite, view};

/// A list of statement triples.
pub type List<'inp, M, V> = Vec<Tagged<M, Triple<'inp, M, V>>>;

/// Shorthand for the type of a triple assertion.
pub type TripleAssertion<'inp, M, V> = Option<Tagged<M, view::Assertion<'inp, M, V>>>;

/// A statement that is optionally surrounded with view assertions to form a Hoare triple.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Triple<'inp, M, V> {
    /// The pre-condition.
    pub pre: TripleAssertion<'inp, M, V>,
    /// The statement.
    pub stm: Tagged<M, Stm<'inp, M, V>>,
    /// The post-condition.
    pub post: TripleAssertion<'inp, M, V>,
}

impl<'inp, M, V> Triple<'inp, M, V> {
    /// Mutably borrows the first assertion in the triple that is not currently populated.
    ///
    /// The main use of this is to fill assertions while parsing.
    #[must_use]
    pub fn first_empty_assertion_mut(&mut self) -> Option<&mut TripleAssertion<'inp, M, V>> {
        if self.pre.is_none() {
            Some(&mut self.pre)
        } else if self.post.is_none() {
            Some(&mut self.post)
        } else {
            None
        }
    }
}

/// The default triple is a no-op with no pre- or post-condition.
impl<'inp, M: Default, V> Default for Triple<'inp, M, V> {
    fn default() -> Self {
        Self {
            pre: None,
            stm: Tagged::with_default(Stm::Nop),
            post: None,
        }
    }
}

/// A statement.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum Stm<'inp, M, V> {
    /// An assignment statement.
    Assign(Assign<'inp, M, V>),
    /// An atomic block statement.
    ///
    /// The semantics of an atomic statement is that all of the statements contained within are
    /// combined into one effective statement for the purposes of verification.  In other words,
    /// there is no interference permitted at any point within.
    Atomic(List<'inp, M, V>),
    /// A non-atomic block statement.
    ///
    /// The semantics of a block statement is that all of the statements contained within are
    /// considered separately but sequentially for the purposes of verification.  In other words,
    /// we permit interference at any point within.
    Block(List<'inp, M, V>),
    /// A call statement.
    ///
    /// The semantics of a call statement is an assert-assume: we assert that we satisfy the
    /// pre-condition of the procedure, and assume the post-condition of the procedure.
    Call(call::Call<'inp, M, V>),
    /// An if-then-else statement.
    Ite(Ite<'inp, M, V>),
    /// A no-operation statement.
    #[default]
    Nop,
}

/// An assignment.
///
/// We reserve the right to add new data to this struct, for instance to capture more advanced forms
/// of assignment.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Assign<'inp, M, V> {
    /// The expression corresponding to the location to receive the value.
    ///
    /// If `None`, then the expression is evaluated and discarded.
    pub lvalue: Option<expr::Tagged<'inp, M, V>>,
    /// The expression corresponding to the value to be assigned.
    pub rvalue: expr::Tagged<'inp, M, V>,
}

/// The default assignment discards 0.
impl<'inp, M: Default, V> Default for Assign<'inp, M, V> {
    fn default() -> Self {
        Self {
            lvalue: None,
            rvalue: Tagged::with_default(expr::Expr::i64(0)),
        }
    }
}

/// Type of if-then-else statements.
pub type Ite<'inp, M, V> = ite::Ite<'inp, M, V, Tagged<M, Box<Stm<'inp, M, V>>>>;
