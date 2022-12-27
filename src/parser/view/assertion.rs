//! The view assertion parser.
//!
//! View assertions are a rich form of expression and, as such, use a Pratt
//! parser similar to that of expressions.

use once_cell::sync::OnceCell;
use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::{Op, PrattParser},
    Span,
};

use super::super::{
    super::language::{
        ast::{view::assertion, Identifier},
        tagged::Spanned,
    },
    call,
    utils::{self, l_infix},
    Rule,
};

static PARSER: OnceCell<PrattParser<Rule>> = OnceCell::new();

/// Initialises the assertion Pratt parser.
fn init() -> PrattParser<Rule> {
    use Rule::*;
    PrattParser::new()
        .op(l_infix(view_join))
        .op(l_infix(view_ite))
        .op(Op::prefix(view_guard))
        .op(Op::postfix(view_iterate))
}

/// Shorthand for the type of pattern produced by the parser.
pub type Assertion<'inp> = assertion::Assertion<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Parses a `pair` into a view assertion (with brackets).
#[must_use]
pub fn parse(pair: Pair<Rule>) -> Assertion {
    utils::match_rule!(pair {
        view_assertion_body => body(pair.into_inner())
    })
}

/// Parses a view assertion body given the `pairs` over its contents.
#[must_use]
pub fn body(pairs: Pairs<Rule>) -> Assertion {
    let parser = PARSER.get_or_init(init);
    parser
        .map_primary(primary)
        .map_infix(infix)
        .map_postfix(postfix)
        .parse(pairs)
}

/// Parses a primary view assertion.
fn primary(pair: Pair<Rule>) -> Assertion {
    utils::match_rule!(pair {
        call => Assertion::Atom(utils::lift_many(pair, call::parse)),
        view_assertion_body => body(pair.into_inner())
    })
}

/// Parses an infix view assertion operator.
///
/// Right now, there is only one infix operator: view join.
fn infix<'inp>(
    lhs: Assertion<'inp>,
    op: Pair<'inp, Rule>,
    rhs: Assertion<'inp>,
) -> Assertion<'inp> {
    utils::match_rule!(op {
        view_join => Assertion::Join(Box::new(lhs), Box::new(rhs))
    })
}

/// Parses a postfix view assertion operator.
fn postfix<'inp>(lhs: Assertion<'inp>, op: Pair<'inp, Rule>) -> Assertion<'inp> {
    utils::match_rule!(op {
        view_iterate => Assertion::Iterated(super::iterate(utils::one_inner(op), Box::new(lhs)))
    })
}
