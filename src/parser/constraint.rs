//! Parsers for constraints.

use pest::{
    iterators::{Pair, Pairs},
    Span,
};

use super::{
    super::language::ast::{constraint, Identifier},
    utils, view, Rule,
};

/// Shorthand for type of constraint declarations returned by this parser.
pub type Decl<'inp> = constraint::Decl<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Shorthand for type of constraint bodies returned by this parser.
pub type Constraint<'inp> = constraint::Constraint<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Shorthand for type of entailment bodies returned by this parser.
pub type Entailment<'inp> = constraint::Entailment<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Parses a constraint declaration from a list of pairs.
#[must_use]
pub fn decl(pairs: Pairs<Rule>) -> Decl {
    utils::match_rules!(pair in pairs, constr : Decl {
        view_pattern => constr.views.push(utils::lift_many(pair, view::pattern)),
        constraint => constr.body = utils::lift_one(pair, parse)
    })
}

/// Parses a constraint body from a pair.
fn parse(pair: Pair<Rule>) -> Constraint {
    match pair.as_rule() {
        Rule::entails_constraint => Constraint::Entails(entailment(utils::one(pair.into_inner()))),
        r => utils::unexpected_rule(r),
    }
}

/// Constructs an entailment body from a pair.
fn entailment(pair: Pair<Rule>) -> Entailment {
    utils::match_rule!(pair {
        expr => Entailment{ entails: utils::lift_many(pair, super::expr::parse) }
    })
}
