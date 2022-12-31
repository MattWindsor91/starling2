//! Parsers for programs, declarations, and procedures.

use pest::{
    iterators::{Pair, Pairs},
    Span,
};

use super::{
    super::language::ast::{program, Identifier},
    call, constraint, stm, utils, var, view, Rule,
};

/// Type of program as parsed by the parser.
pub type Program<'inp> = program::Program<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Shorthand for the type of declarations returned by this parser.
pub type Decl<'inp> = program::Decl<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Parses `pairs` as a program.
#[must_use]
pub fn parse(pairs: Pairs<Rule>) -> Program {
    utils::match_rules!(pair in pairs, prog: Program {
        identifier => prog.name = utils::spanned_id(&pair),
        decl => prog.decls.push(utils::lift_one(pair, decl)),
        EOI => ()
    })
}

/// Parses `pair` as a decl.
#[must_use]
pub fn decl(pair: Pair<Rule>) -> Decl {
    utils::match_rule!(pair {
        constraint_decl => Decl::Constraint(constraint::decl(pair.into_inner())),
        procedure_decl => Decl::Procedure(procedure(pair.into_inner())),
        var_decl => Decl::Var(var::decl(pair.into_inner())),
        view_decl => Decl::View(view::decl::parse(pair.into_inner()))
    })
}

/// Shorthand for the type of procedures returned by this parser.
pub type Procedure<'inp> = program::Procedure<'inp, Option<Span<'inp>>, Identifier<'inp>>;

#[must_use]
fn procedure(pairs: Pairs<Rule>) -> program::Procedure<Option<Span>, Identifier> {
    utils::match_rules!(pair in pairs, proc : Procedure {
        prototype => proc.prototype = utils::lift_many(pair, call::prototype),
        block => proc.body = stm::block(utils::one_inner(pair))
    })
}
