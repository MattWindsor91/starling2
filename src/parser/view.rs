//! Parser for views and view atoms.

pub mod assertion;
pub mod decl;
pub mod pattern;

use pest::{iterators::Pair, Span};

use super::{
    super::language::ast::{view, Identifier},
    expr, utils, Rule,
};

/// Shorthand for the type of iterated views/atoms parsed by `iterate`.
pub type Iterated<'inp, T> = view::Iterated<'inp, Option<Span<'inp>>, Identifier<'inp>, T>;

/// Parses a view iterator from `pair`, wrapping `item` within it.
#[must_use]
pub fn iterate<T>(pair: Pair<Rule>, item: T) -> Iterated<T> {
    utils::match_rule!(pair {
        expr => Iterated{item, iterator: utils::lift_many(pair, expr::parse)}
    })
}
