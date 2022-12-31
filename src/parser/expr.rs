//! The expression parser.

use once_cell::sync::OnceCell;
use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::{Op, PrattParser},
    Span,
};

use super::{
    super::language::ast::{expr, Identifier},
    utils::{self, l_infix},
    Rule,
};

static PARSER: OnceCell<PrattParser<Rule>> = OnceCell::new();

/// Initialises the Pratt parser.
fn init() -> PrattParser<Rule> {
    PrattParser::new()
        .op(l_infix(Rule::eq)
            | l_infix(Rule::not_eq)
            | l_infix(Rule::less)
            | l_infix(Rule::less_eq)
            | l_infix(Rule::greater)
            | l_infix(Rule::greater_eq))
        .op(l_infix(Rule::add) | l_infix(Rule::sub) | l_infix(Rule::or))
        .op(l_infix(Rule::mul)
            | l_infix(Rule::div)
            | l_infix(Rule::int_div)
            | l_infix(Rule::modulus)
            | l_infix(Rule::and))
        .op(l_infix(Rule::implies) | l_infix(Rule::iff))
        .op(Op::prefix(Rule::not) | Op::prefix(Rule::minus) | Op::prefix(Rule::plus))
        .op(Op::postfix(Rule::subscript))
}

/// Shorthand for type of expressions returned by this parser.
pub type Expr<'inp> = expr::Expr<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Parses an expression.
///
/// The expression parser is a Pest-based Pratt parser, so it takes a pairs iterator rather than
/// a single pair.
#[must_use]
pub fn parse(pairs: Pairs<Rule>) -> Expr {
    let parser = PARSER.get_or_init(init);
    parser
        .map_primary(primary)
        .map_prefix(|op, rhs| Expr::uop(prefix_op(&op), rhs))
        .map_postfix(|lhs, op| Expr::uop(postfix_op(&op), lhs))
        .map_infix(|lhs, op, rhs| Expr::bop(lhs, infix_op(&op), rhs))
        .parse(pairs)
}

fn primary(primary: Pair<Rule>) -> Expr {
    utils::match_rule!(primary {
        identifier => Expr::Var(utils::spanned_id(&primary)),
        literal => literal(utils::one(primary.into_inner())),
        expr => parse(primary.into_inner())
    })
}

/// Parses prefix operators.
fn prefix_op(op: &Pair<Rule>) -> expr::Uop {
    utils::match_rule!(op {
        minus => expr::Uop::Minus,
        not => expr::Uop::Not,
        plus => expr::Uop::Plus
    })
}

/// Parses postfix operators.
fn postfix_op(op: &Pair<Rule>) -> expr::Uop {
    utils::match_rule!(op {
        deref => expr::Uop::Deref
    })
}

/// Parses infix operators.
fn infix_op(pair: &Pair<Rule>) -> expr::Bop {
    use expr::bop::{Arith, Bool, Bop, Rel};
    utils::match_rule!(pair {
        add => Bop::Arith(Arith::Add),
        div => Bop::Arith(Arith::Div),
        modulus => Bop::Arith(Arith::Modulus),
        int_div => Bop::Arith(Arith::IntDiv),
        sub => Bop::Arith(Arith::Sub),
        mul => Bop::Arith(Arith::Mul),
        and => Bop::Bool(Bool::And),
        iff => Bop::Bool(Bool::Iff),
        implies => Bop::Bool(Bool::Implies),
        or => Bop::Bool(Bool::Or),
        eq => Bop::Rel(Rel::Eq),
        not_eq => Bop::Rel(Rel::NotEq),
        less => Bop::Rel(Rel::Less),
        less_eq => Bop::Rel(Rel::LessEq),
        greater => Bop::Rel(Rel::Greater),
        greater_eq => Bop::Rel(Rel::GreaterEq)
    })
}

/// Parses a literal expression.
fn literal(pair: Pair<Rule>) -> Expr {
    Expr::Literal(utils::spanned(
        pair.as_span(),
        utils::match_rule!(pair {
            int_literal => expr::Literal::Int(int(pair.as_str())),
            bool_literal => expr::Literal::Bool(bool(&utils::one_inner(pair)))
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

/// Parses a Boolean.
///
fn bool(pair: &Pair<Rule>) -> bool {
    utils::match_rule!(pair {
        true_literal => true,
        false_literal => false
    })
}
