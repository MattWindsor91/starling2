//! Parsers for statements and related constructs.

use pest::{
    iterators::{Pair, Pairs},
    Span,
};

use super::{
    super::language::ast::{stm, Identifier},
    call, utils, Rule,
};

/// Shorthand for type of statement lists returned by the parser.
pub type List<'inp> = stm::List<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Shorthand for type of statement triples returned by the parser.
pub type Triple<'inp> = stm::Triple<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Shorthand for type of statements returned by the parser.
pub type Stm<'inp> = stm::Stm<'inp, Option<Span<'inp>>, Identifier<'inp>>;

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
    let asst = utils::lift_one(pair, super::view::assertion::parse);
    if triple.pre.is_none() {
        triple.pre = Some(asst)
    } else if triple.post.is_none() {
        triple.post = Some(asst)
    } else {
        unreachable!("should be only two views in a triple");
    }
}

/// Parses `pair` as a statement.
#[must_use]
pub fn parse(pair: Pair<Rule>) -> Stm {
    utils::match_rule!(pair {
        atomic_stm => Stm::Atomic(atomic(utils::one_inner(pair))),
        call => Stm::Call(call::parse(pair.into_inner())),
        nop_stm => Stm::Nop
    })
}

/// Parses `pair` as an atomic statement.
#[must_use]
pub fn atomic(pair: Pair<Rule>) -> List {
    utils::match_rule!(pair {
        stm_list => list(pair.into_inner())
    })
}
