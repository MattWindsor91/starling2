//! Utilities for the parser.

use itertools::Itertools;
use pest::iterators::{Pair, Pairs};

use super::Spanned;

/// Creates a spanned identifier from its pair.
#[must_use]
pub fn spanned_id(pair: Pair<super::Rule>) -> Spanned<super::ast::Identifier> {
    Spanned::new(
        Some(pair.as_span()),
        super::ast::Identifier::from(pair.as_str()),
    )
}

/// Wraps a parser that consumes all of the inner pairs of `pair`.
///
/// We use the parent `pair` to discern the span for the whole alternative and lift the result with
/// that span.
pub fn lift_many<'inp, T>(
    pair: Pair<'inp, super::Rule>,
    parser: fn(Pairs<'inp, super::Rule>) -> T,
) -> Spanned<'inp, T> {
    Spanned::new(Some(pair.as_span()), parser(pair.into_inner()))
}

/// Given a pair that wraps one of multiple alternatives, applies `parser` to that alternative.
///
/// We use the parent `pair` to discern the span for the whole alternative and lift the result with
/// that span.
pub fn lift_one<'inp, T>(
    pair: Pair<'inp, super::Rule>,
    parser: fn(Pair<'inp, super::Rule>) -> T,
) -> Spanned<'inp, T> {
    Spanned::new(Some(pair.as_span()), parser(one(pair.into_inner())))
}

/// Enforces that exactly one pair exists in `pairs`, and extracts it.
///
/// # Panics
///
/// Panics if there is more or less than one pair in `pairs`.
pub fn one(pairs: Pairs<super::Rule>) -> Pair<super::Rule> {
    pairs
        .exactly_one()
        .expect("expected exactly one match here")
}

/// Panics with an unexpected rule.
///
/// # Panics
///
/// Always.
pub fn unexpected_rule(rule: super::Rule) -> ! {
    unreachable!("parser should not have accepted {:?} here", rule)
}
