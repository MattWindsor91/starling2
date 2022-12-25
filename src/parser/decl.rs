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
    call, utils, Rule,
};

/// Parses a pair as a decl.
pub fn parse(pair: Pair<Rule>) -> Decl<Option<Span>, Identifier> {
    let rule = pair.as_rule();
    let inner = pair.into_inner();
    match rule {
        Rule::constraint_decl => Decl::Constraint(constraint(inner)),
        Rule::procedure_decl => Decl::Procedure(procedure(inner)),
        Rule::view_decl => Decl::View(view(inner)),
        r => utils::unexpected_rule(r),
    }
}

fn constraint(pairs: Pairs<Rule>) -> Constraint<Option<Span>, Identifier> {
    let mut constr = Constraint {
        view: view::Pattern::default(),
        body: constraint::Body::Expr(constraint::Expr {
            entails: Expr::bool(true),
        }),
    };

    // TODO

    constr
}

fn procedure(pairs: Pairs<Rule>) -> decl::Procedure<Option<Span>, Identifier> {
    pairs.fold(decl::Procedure::default(), |mut proc, pair| {
        match pair.as_rule() {
            Rule::prototype => proc.prototype = utils::lift_many(pair, call::prototype),
            Rule::statement_list => proc.body = statement_list(pair.into_inner()),
            r => utils::unexpected_rule(r),
        };
        proc
    })
}

fn statement_list(
    pairs: Pairs<Rule>,
) -> Vec<Spanned<super::ast::StatementWithViews<Option<Span>, Identifier>>> {
    // TODO
    vec![]
}

fn view(pairs: Pairs<Rule>) -> decl::View<Option<Span>, Identifier> {
    let mut view = decl::View { elements: vec![] };

    // TODO

    view
}
