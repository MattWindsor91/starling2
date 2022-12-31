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
    super::language::ast::{view::assertion, Identifier},
    call, expr,
    utils::{self, l_infix},
    Rule,
};

static PARSER: OnceCell<PrattParser<Rule>> = OnceCell::new();

/// Initialises the assertion Pratt parser.
fn init() -> PrattParser<Rule> {
    PrattParser::new()
        .op(l_infix(Rule::view_join))
        .op(l_infix(Rule::view_ite))
        .op(Op::prefix(Rule::view_guard))
        .op(Op::postfix(Rule::view_iterate))
}

/// Shorthand for the type of pattern produced by the parser.
pub type Assertion<'inp> = assertion::Assertion<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Parses a view assertion body given the `pairs` over its contents.
#[must_use]
pub fn parse(pairs: Pairs<Rule>) -> Assertion {
    let parser = PARSER.get_or_init(init);
    parser
        .map_primary(primary)
        .map_infix(|lhs, op, rhs| infix(lhs, &op, rhs))
        .map_postfix(postfix)
        .parse(pairs)
}

/// Parses a primary view assertion.
fn primary(pair: Pair<Rule>) -> Assertion {
    utils::match_rule!(pair {
        call => Assertion::Atom(utils::lift_many(pair, call::parse)),
        expr => Assertion::Local(utils::lift_many(pair, expr::parse)),
        empty_view => Assertion::Emp,
        view_assertion => parse(pair.into_inner())
    })
}

/// Parses an infix view assertion operator.
///
/// Right now, there is only one infix operator: view join.
fn infix<'inp>(
    lhs: Assertion<'inp>,
    op: &Pair<'inp, Rule>,
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
