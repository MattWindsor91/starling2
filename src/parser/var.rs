//! Parsers for variable declarations.

use pest::{
    iterators::{Pair, Pairs},
    Span,
};

use super::{
    super::language::{ast::var, var::Scope},
    call, utils, Rule,
};

/// Shorthand for the type of decl returned by `decl`.
pub type Decl<'inp> = var::Decl<'inp, Option<Span<'inp>>, var::Identifier<'inp>>;

/// Parses `pairs` as a variable declaration.
#[must_use]
pub fn decl(pairs: Pairs<Rule>) -> Decl {
    utils::match_rules!(pair in pairs, dec: Decl {
        var_scope => dec.scope = utils::lift_one(pair, |p| scope(&p)),
        parameter => dec.vars.push(utils::lift_many(pair, call::parameter))
    })
}

/// Parses `pair` as a scope.
#[must_use]
fn scope(pair: &Pair<Rule>) -> Scope {
    utils::match_rule!(pair {
        shared_scope => Scope::Shared,
        thread_scope => Scope::Thread
    })
}
