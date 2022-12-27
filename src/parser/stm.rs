//! Parsers for statements and related constructs.

use pest::{iterators::Pairs, Span};

use super::{
    super::language::ast::{stm, Identifier},
    utils, Rule,
};

/// Shorthand for type of statement lists returned by the parser.
pub type List<'inp> = stm::List<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Shorthand for type of statement triples returned by the parser.
pub type Triple<'inp> = stm::Triple<'inp, Option<Span<'inp>>, Identifier<'inp>>;

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
        view_assertion => {
            let asst = utils::lift_many(pair, super::view::assertion);
            if triple.pre.is_none() {
                triple.pre = Some(asst)
            } else if triple.post.is_none() {
                triple.post = Some(asst)
            } else {
                unreachable!("should be only two views in a triple");
            }
        },
        stm => () // TODO
    })
}
