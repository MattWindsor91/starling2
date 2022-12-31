//! Parsers for declarations.

use pest::{
    iterators::{Pair, Pairs},
    Span,
};

use super::{
    super::language::ast::{decl, Identifier},
    call, constraint, stm, utils, view, Rule,
};

/// Shorthand for the type of declarations returned by this parser.
pub type Decl<'inp> = decl::Decl<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Parses a pair as a decl.
pub fn parse(pair: Pair<Rule>) -> Decl {
    utils::match_rule!(pair {
        constraint_decl => Decl::Constraint(constraint::decl(pair.into_inner())),
        procedure_decl => Decl::Procedure(procedure(pair.into_inner())),
        view_decl => Decl::View(view::decl::parse(pair.into_inner()))
    })
}

/// Shorthand for the type of procedures returned by this parser.
pub type Procedure<'inp> = decl::Procedure<'inp, Option<Span<'inp>>, Identifier<'inp>>;

#[must_use]
fn procedure(pairs: Pairs<Rule>) -> decl::Procedure<Option<Span>, Identifier> {
    utils::match_rules!(pair in pairs, proc : Procedure {
        prototype => proc.prototype = utils::lift_many(pair, call::prototype),
        stm_list => proc.body = stm::list(pair.into_inner())
    })
}
