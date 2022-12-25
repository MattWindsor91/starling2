//! Parsers for call-related constructs.

use pest::{
    iterators::{Pair, Pairs},
    Span,
};

use super::{
    super::language::{
        ast::{call, constraint, decl, view, Call, Constraint, Decl, Expr, Identifier},
        tagged::Spanned,
    },
    utils, Rule,
};

/// Parses a `pair` representing a function prototype.
pub fn prototype(pairs: Pairs<Rule>) -> call::Prototype<Option<Span>, Identifier> {
    pairs.fold(call::Prototype::default(), |mut proto, pair| {
        match pair.as_rule() {
            Rule::identifier => proto.name = utils::spanned_id(pair),
            r => utils::unexpected_rule(r),
        };
        proto
    })
}
