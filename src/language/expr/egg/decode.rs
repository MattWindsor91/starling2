//! Decode an egg rec-expr into a PVC expression.

use egg::{Id, Symbol};

use super::{
    super::{super::tagged::Tagged, bop, uop},
    Term,
};

/// The decoder produces expressions with no metadata, and symbolic variables.
pub type Expr = super::super::Expr<(), Symbol>;

/// Converts egg rec-expr `e` into a PVC expression.
#[must_use]
pub fn expr(e: &super::Expr, top_id: Id) -> Expr {
    match &e[top_id] {
        Term::Add([l, r]) => binary_op(e, bop::Arith::Add, *l, *r),
        Term::Sub([l, r]) => binary_op(e, bop::Arith::Sub, *l, *r),
        Term::Mul([l, r]) => binary_op(e, bop::Arith::Mul, *l, *r),
        Term::Div([l, r]) => binary_op(e, bop::Arith::Div, *l, *r),
        Term::Modulus([l, r]) => binary_op(e, bop::Arith::Modulus, *l, *r),
        Term::And([l, r]) => binary_op(e, bop::Bool::And, *l, *r),
        Term::Or([l, r]) => binary_op(e, bop::Bool::Or, *l, *r),
        Term::Implies([l, r]) => binary_op(e, bop::Bool::Implies, *l, *r),
        Term::Iff([l, r]) => binary_op(e, bop::Bool::Iff, *l, *r),
        Term::Less([l, r]) => binary_op(e, bop::Rel::Less, *l, *r),
        Term::LessEq([l, r]) => binary_op(e, bop::Rel::LessEq, *l, *r),
        Term::Eq([l, r]) => binary_op(e, bop::Rel::Eq, *l, *r),
        Term::NotEq([l, r]) => binary_op(e, bop::Rel::NotEq, *l, *r),
        Term::GreaterEq([l, r]) => binary_op(e, bop::Rel::GreaterEq, *l, *r),
        Term::Greater([l, r]) => binary_op(e, bop::Rel::Greater, *l, *r),
        Term::Minus(x) => unary_op(e, uop::Uop::Minus, *x),
        Term::Plus(x) => unary_op(e, uop::Uop::Plus, *x),
        Term::Deref(x) => unary_op(e, uop::Uop::Deref, *x),
        Term::Not(x) => unary_op(e, uop::Uop::Not, *x),
        Term::Constant(k) => Expr::Literal(Tagged::with_default(k.clone())),
        Term::Var(v) => Expr::Var(Tagged::with_default(*v)),
    }
}

fn binary_op(e: &super::Expr, op: impl Into<bop::Bop>, lhs: Id, rhs: Id) -> Expr {
    Expr::bop(expr(e, lhs), op.into(), expr(e, rhs))
}

fn unary_op(e: &super::Expr, op: uop::Uop, inner: Id) -> Expr {
    Expr::uop(op, expr(e, inner))
}
