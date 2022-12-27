//! Utilities for the parser.

use itertools::Itertools;
use pest::{
    iterators::{Pair, Pairs},
    Span,
};

use super::Spanned;

/// Repeatedly matches rules in a pairs iterator against a list of patterns, using them to populate
/// an initially-default syntactic construct; finally returns that construct.
///
/// This macro is syntactic sugar over a match, fold, and default.
macro_rules! match_rules {
    ($pair:ident in $pairs:expr, $binder:ident : $node_type:ty { $($rule:ident => $rule_expr:expr),* }) => {
        $pairs.fold(<$node_type>::default(), |mut $binder, $pair| {
            match $pair.as_rule() {
                $(crate::parser::Rule::$rule => $rule_expr),*,
                r => crate::parser::utils::unexpected_rule(r),
            };
            $binder
        })
    };
}
pub(crate) use match_rules;

/// Repeatedly matches rules in a pairs iterator against a list of patterns, using them to populate
/// an initially-default syntactic construct; finally returns that construct.
///
/// This macro is syntactic sugar over a match, fold, and default.
macro_rules! match_rule {
    ($pair:ident { $($rule:ident => $rule_expr:expr),* }) => {
        match $pair.as_rule() {
            $(crate::parser::Rule::$rule => $rule_expr),*,
            r => crate::parser::utils::unexpected_rule(r),
        }
    };
}
pub(crate) use match_rule;

/// Wraps an item in a span.
#[must_use]
pub fn spanned<T>(span: Span, item: T) -> Spanned<T> {
    Spanned::new(Some(span), item)
}

/// Creates a spanned identifier from its pair.
#[must_use]
pub fn spanned_id(pair: Pair<super::Rule>) -> Spanned<super::ast::Identifier> {
    spanned(pair.as_span(), super::ast::Identifier::from(pair.as_str()))
}

/// Wraps a parser that consumes all of the inner pairs of `pair`.
///
/// We use the parent `pair` to discern the span for the whole alternative and lift the result with
/// that span.
#[must_use]
pub fn lift_many<'inp, T>(
    pair: Pair<'inp, super::Rule>,
    parser: fn(Pairs<'inp, super::Rule>) -> T,
) -> Spanned<'inp, T> {
    spanned(pair.as_span(), parser(pair.into_inner()))
}

/// Given a pair that wraps one of multiple alternatives, applies `parser` to that alternative.
///
/// We use the parent `pair` to discern the span for the whole alternative and lift the result with
/// that span.
#[must_use]
pub fn lift_one<'inp, T>(
    pair: Pair<'inp, super::Rule>,
    parser: fn(Pair<'inp, super::Rule>) -> T,
) -> Spanned<'inp, T> {
    spanned(pair.as_span(), parser(one(pair.into_inner())))
}

/// Enforces that exactly one pair exists in `pairs`, and extracts it.
///
/// # Panics
///
/// Panics if there is more or less than one pair in `pairs`.
#[must_use]
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
#[inline]
pub fn unexpected_rule(rule: super::Rule) -> ! {
    unreachable!("parser should not have accepted {:?} here", rule)
}
