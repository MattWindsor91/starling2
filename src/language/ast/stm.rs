//! Statements.

use super::{super::tagged::Tagged, call, view};
use crate::language::ast::view::assertion;

/// A list of statement triples.
pub type List<'inp, M, V> = Vec<Tagged<M, Triple<'inp, M, V>>>;

/// A statement that is optionally surrounded with view assertions to form a Hoare triple.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Triple<'inp, M, V> {
    /// The precondition.
    pub pre: Option<Tagged<M, assertion::Assertion<'inp, M, V>>>,
    /// The statement.
    pub stm: Tagged<M, Stm<'inp, M, V>>,
    /// The postcondition.
    pub post: Option<Tagged<M, assertion::Assertion<'inp, M, V>>>,
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
    /// An atomic statement.
    ///
    /// The semantics of an atomic statement is that all of the statements contained within are
    /// combined into one effective statement for the purposes of verification.  In other words,
    /// there is no interference permitted at any point within.
    Atomic(List<'inp, M, V>),
    /// A call statement.
    ///
    /// The semantics of a call statement is an assert-assume: we assert that we satisfy the
    /// pre-condition of the procedure, and assume the post-condition of the procedure.
    Call(call::Call<'inp, M, V>),
    /// A no-operation statement.
    #[default]
    Nop,
}
