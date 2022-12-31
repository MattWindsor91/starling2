//! Parsers for statements and related constructs.

use pest::{
    iterators::{Pair, Pairs},
    Span,
};
use std::mem;

use super::{
    super::language::ast::{ite, stm, Identifier},
    call, expr, utils, Rule,
};

/// Shorthand for type of statement lists returned by the parser.
pub type List<'inp> = stm::List<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Shorthand for type of statement triples returned by the parser.
pub type Triple<'inp> = stm::Triple<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Shorthand for type of statements returned by the parser.
pub type Stm<'inp> = stm::Stm<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Shorthand for type of assignments returned by the parser.
pub type Assign<'inp> = stm::Assign<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Shorthand for type of if-then-else returned by the parser.
pub type Ite<'inp> = stm::Ite<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Shorthand for type of condition returned by the parser.
pub type Condition<'inp> = ite::Condition<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Parses `pairs` as a list of triples.
#[must_use]
pub fn list(pairs: Pairs<Rule>) -> List {
    utils::match_rules!(pair in pairs, triples: List {
        stm_triple => triples.push(utils::lift_many(pair, triple))
    })
}

/// Parses `pairs` as a triple.
#[must_use]
pub fn triple(pairs: Pairs<Rule>) -> Triple {
    utils::match_rules!(pair in pairs, triple: Triple {
        view_assertion => triple_view(&mut triple, pair),
        stm => triple.stm = utils::lift_one(pair, parse)
    })
}

/// Places the assertion parsed from `pair` in the first slot in `triple` that is available.
///
/// # Panics
///
/// Panics if this is the third assertion we have parsed for `triple` (there should be only two).
fn triple_view<'i>(triple: &mut Triple<'i>, pair: Pair<'i, Rule>) {
    let dst = triple
        .first_empty_assertion_mut()
        .expect("should be only two views in a triple");
    *dst = Some(utils::lift_many(pair, super::view::assertion::parse));
}

/// Parses `pair` as a statement.
#[must_use]
pub fn parse(pair: Pair<Rule>) -> Stm {
    utils::match_rule!(pair {
        atomic_stm => Stm::Atomic(block(utils::one_inner(pair))),
        assign => Stm::Assign(assign(pair.into_inner())),
        block => Stm::Block(block(utils::one_inner(pair))),
        call => Stm::Call(call::parse(pair.into_inner())),
        ite_stm => Stm::Ite(ite(pair.into_inner())),
        nop_stm => Stm::Nop
    })
}

/// Parses `pairs` as an assign statement.
#[must_use]
pub fn assign(pairs: Pairs<Rule>) -> Assign {
    // Expecting two expressions: lvalue and rvalue.
    utils::match_rules!(pair in pairs, asn : Assign {
        expr => {
            // We don't check to see if we're receiving more expressions than wanted here, as it'd
            // complicate either the parser or the AST to do so.
            let x = utils::lift_many(pair, expr::parse);
            if asn.lvalue.is_none() {
                // The first expression seen is the lvalue.
                asn.lvalue = Some(x);
            } else {
                // The second expression seen is the rvalue.
                asn.rvalue = x;
            }
        }
    })
}

/// Parses `pair` as a block statement.
#[must_use]
pub fn block(pair: Pair<Rule>) -> List {
    utils::match_rule!(pair {
        stm_list => list(pair.into_inner())
    })
}

/// Parses `pairs` as an if-then-else statement.
#[must_use]
fn ite(pairs: Pairs<Rule>) -> Ite {
    let mut need_true = true;
    utils::match_rules!(pair in pairs, stm: Ite {
        condition => stm.cond = utils::lift_one(pair, condition),
        stm => {
            let next_branch = mem::replace(&mut need_true, false);
            *stm.branch_mut(next_branch) = utils::lift_one(pair, parse).map(Box::new);
        }
    })
}

/// Parses `pair` as a block statement.
#[must_use]
fn condition(pair: Pair<Rule>) -> Condition {
    utils::match_rule!(pair {
        nondeterminism => Condition::Nondeterministic,
        expr => Condition::Deterministic(expr::parse(pair.into_inner()))
    })
}
