//! Parser for view declarations.

use pest::{iterators::Pairs, Span};

use super::super::{
    super::language::ast::{view::decl, Identifier},
    call, utils, Rule,
};

/// Shorthand for type of declarations returned by `parse`.
pub type Decl<'inp> = decl::Decl<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Parses `pairs` into a view declaration.
#[must_use]
pub fn parse(pairs: Pairs<Rule>) -> decl::Decl<Option<Span>, Identifier> {
    utils::match_rules!(pair in pairs, views: Decl {
        // We parse view prototypes in the same way as procedure prototypes.
        prototype => views.contents.push(utils::lift_many(pair, call::prototype))
    })
}
