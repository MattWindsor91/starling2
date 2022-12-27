//! Utilities for the parser.

use itertools::Itertools;
use pest::{
    iterators::{Pair, Pairs},
    Span,
};

use super::{super::language::tagged::Spanned, Rule};

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

//
// Spans
//

/// Wraps an item in a span.
#[must_use]
pub fn spanned<T>(span: Span, item: T) -> Spanned<T> {
    Spanned::new(Some(span), item)
}

/// Creates a spanned identifier from its pair.
#[must_use]
pub fn spanned_id(pair: Pair<Rule>) -> Spanned<super::ast::Identifier> {
    spanned(pair.as_span(), super::ast::Identifier::from(pair.as_str()))
}

//
// Lifting parsers
//

/// Wraps a parser that consumes all of the inner pairs of `pair`.
///
/// We use the parent `pair` to discern the span for the whole alternative and lift the result with
/// that span.
#[must_use]
pub fn lift_many<'inp, T>(
    pair: Pair<'inp, Rule>,
    parser: fn(Pairs<'inp, Rule>) -> T,
) -> Spanned<'inp, T> {
    spanned(pair.as_span(), parser(pair.into_inner()))
}

/// Given a pair that wraps one of multiple alternatives, applies `parser` to that alternative.
///
/// We use the parent `pair` to discern the span for the whole alternative and lift the result with
/// that span.
#[must_use]
pub fn lift_one<'inp, T>(
    pair: Pair<'inp, Rule>,
    parser: fn(Pair<'inp, Rule>) -> T,
) -> Spanned<'inp, T> {
    spanned(pair.as_span(), parser(one_inner(pair)))
}

//
// There can be only one
//

/// Enforces that exactly one pair exists in the inner pairs of `pair`, and extracts it.
///
/// # Panics
///
/// Panics if there is more or less than one pair in `pair.into_inner()`.
#[must_use]
pub fn one_inner(pair: Pair<Rule>) -> Pair<Rule> {
    one(pair.into_inner())
}

/// Enforces that exactly one pair exists in `pairs`, and extracts it.
///
/// # Panics
///
/// Panics if there is more or less than one pair in `pairs`.
#[must_use]
pub fn one(pairs: Pairs<Rule>) -> Pair<Rule> {
    pairs
        .exactly_one()
        .expect("expected exactly one match here")
}

//
// Error conditions
//

/// Panics with an unexpected rule.
///
/// # Panics
///
/// Always.
#[inline]
pub fn unexpected_rule(rule: Rule) -> ! {
    unreachable!("parser should not have accepted {:?} here", rule)
}

//
// Pratt parsing
//

/// Shorthand for a left-infix rule.
#[must_use]
pub fn l_infix(rule: Rule) -> pest::pratt_parser::Op<Rule> {
    pest::pratt_parser::Op::infix(rule, pest::pratt_parser::Assoc::Left)
}
