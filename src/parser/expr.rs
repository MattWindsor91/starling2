//! The expression parser.

use crate::language::ast::expr::bop::Rel::GreaterEq;
use once_cell::sync::OnceCell;
use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::{Assoc, Op, PrattParser},
    Span,
};

use super::{
    super::language::ast::{expr, Identifier},
    utils, Rule,
};

static PARSER: OnceCell<PrattParser<Rule>> = OnceCell::new();

/// Initialises the Pratt parser.
fn init() -> PrattParser<Rule> {
    use Rule::*;
    PrattParser::new()
        .op(l_infix(eq)
            | l_infix(not_eq)
            | l_infix(less)
            | l_infix(less_eq)
            | l_infix(greater)
            | l_infix(greater_eq))
        .op(l_infix(add) | l_infix(sub) | l_infix(or))
        .op(l_infix(mul) | l_infix(div) | l_infix(int_div) | l_infix(modulus) | l_infix(and))
        .op(l_infix(implies) | l_infix(iff))
        .op(Op::prefix(not) | Op::prefix(neg) | Op::prefix(pos))
        .op(Op::postfix(subscript))
}

/// Shorthand for type of expressions returned by this parser.
pub type Expr<'inp> = expr::Expr<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Parses an expression.
///
/// The expression parser is a Pest-based Pratt parser, and so it takes a pairs iterator rather than
/// a single pair.
#[must_use]
pub fn parse(pairs: Pairs<Rule>) -> Expr {
    let parser = PARSER.get_or_init(init);
    parser
        .map_primary(primary)
        .map_prefix(|op, rhs| match op.as_rule() {
            r => utils::unexpected_rule(r),
        })
        .map_postfix(|lhs, op| match op.as_rule() {
            r => utils::unexpected_rule(r),
        })
        .map_infix(infix)
        .parse(pairs)
}

fn primary(primary: Pair<Rule>) -> Expr {
    utils::match_rule!(primary {
        identifier => Expr::Var(utils::spanned_id(primary)),
        literal => literal(utils::one(primary.into_inner())),
        expr => parse(primary.into_inner())
    })
}

fn infix<'inp>(lhs: Expr<'inp>, op: Pair<Rule>, rhs: Expr<'inp>) -> Expr<'inp> {
    Expr::Bop {
        lhs: Box::new(lhs),
        op: infix_op(op),
        rhs: Box::new(rhs),
    }
}

/// Parses infix operators.
fn infix_op(pair: Pair<Rule>) -> expr::Bop {
    use expr::bop::Arith::*;
    use expr::bop::Bool::*;
    use expr::bop::Rel::*;
    use expr::Bop::*;
    utils::match_rule!(pair {
        add => Arith(Add),
        div => Arith(Div),
        modulus => Arith(Modulus),
        sub => Arith(Sub),
        mul => Arith(Mul),
        and => Bool(And),
        iff => Bool(Iff),
        implies => Bool(Implies),
        or => Bool(Or),
        eq => Rel(Eq),
        not_eq => Rel(NotEq),
        less => Rel(Less),
        less_eq => Rel(LessEq),
        greater => Rel(Greater),
        greater_eq => Rel(GreaterEq)
    })
}

fn l_infix(rule: Rule) -> Op<Rule> {
    Op::infix(rule, Assoc::Left)
}

/*
or else, and then, Lowest
=, <>, <, <=, >, >=, in
|, !, +, -, or,
*, /, div, mod, and, &
~, not,	Highest
*/

/// Parses a literal expression.
fn literal(pair: Pair<Rule>) -> Expr {
    Expr::Literal(utils::spanned(
        pair.as_span(),
        utils::match_rule!(pair {
            int_literal => expr::Literal::Int(int(pair.as_str())),
            bool_literal => expr::Literal::Bool(bool(pair.as_str()))
        }),
    ))
}

/// Parses an integer.
///
/// # Panics
///
/// Panics if the upstream parser sent us an integer literal that is badly formed.
fn int(inp: &str) -> expr::literal::Int {
    match inp.parse() {
        Ok(i) => expr::literal::Int::I64(i),
        Err(err)
            if matches!(
                err.kind(),
                std::num::IntErrorKind::NegOverflow | std::num::IntErrorKind::PosOverflow
            ) =>
        {
            expr::literal::Int::Big(inp)
        }
        Err(_) => unreachable!(
            "parser should have disallowed erroneous integer input {:?}",
            inp
        ),
    }
}

/// Parses a Boolean..
///
/// # Panics
///
/// Panics if the upstream parser sent us a Boolean literal that is badly formed.
fn bool(inp: &str) -> bool {
    if inp.eq_ignore_ascii_case("true") {
        true
    } else if inp.eq_ignore_ascii_case("false") {
        false
    } else {
        unreachable!(
            "parser should have disallowed erroneous boolean input {:?}",
            inp
        )
    }
}
