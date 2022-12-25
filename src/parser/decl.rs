//! Parsers for declarations.

use pest::{
    iterators::{Pair, Pairs},
    Span,
};

use super::{
    super::language::{
        ast::{constraint, decl, view, Call, Constraint, Decl, Expr, Identifier},
        tagged::Spanned,
    },
    utils, Rule,
};

/// Parses a `declaration` rule as a spanned decl.
pub fn spanned(pair: Pair<Rule>) -> Spanned<Decl<Span, Identifier>> {
    assert!(matches!(pair.as_rule(), Rule::decl));

    Spanned::new(pair.as_span(), parse(utils::one(pair.into_inner())))
}

/// Parses a pair as a decl.
pub fn parse(pair: Pair<Rule>) -> Decl<Span, Identifier> {
    let rule = pair.as_rule();
    let inner = pair.into_inner();
    match rule {
        Rule::constraint_decl => Decl::Constraint(constraint(inner)),
        Rule::procedure_decl => Decl::Procedure(procedure(inner)),
        Rule::view_decl => Decl::View(view(inner)),
        r => utils::unexpected_rule(r),
    }
}

fn constraint(pairs: Pairs<Rule>) -> Constraint<Span, Identifier> {
    let mut constr = Constraint {
        view: view::Pattern::default(),
        body: constraint::Body::Expr(constraint::Expr {
            entails: Expr::bool(true),
        }),
    };

    // TODO

    constr
}

fn procedure(pairs: Pairs<Rule>) -> decl::Procedure<Span, Identifier> {
    let mut proc = decl::Procedure {
        prototype: Call {
            name: Identifier::default(),
            args: vec![],
        },
        body: vec![],
    };

    // TODO

    proc
}

fn view(pairs: Pairs<Rule>) -> decl::View<Span, Identifier> {
    let mut view = decl::View { elements: vec![] };

    // TODO

    view
}
