//! Encoding a PVC expression into an `egg` rec-expr.
//!
//! Encoding consists of recursively adding parts of an expression to a work-in-progress `egg`
//! expression, with `add_expr` as the

use egg::{Id, Symbol};

use super::{
    super::{bop, Constant, Expr, Uop},
    Term,
};

/// Adds a PVC expression to an egg rec-expr.
#[must_use]
pub fn expr<M, V: Clone + Into<Symbol>>(dest: &mut super::Expr, expr: &Expr<M, V>) -> Id {
    match expr {
        Expr::Literal(k) => constant(dest, k.item.clone()),
        Expr::Var(v) => dest.add(Term::Var(v.item.clone().into())),
        Expr::Bop { lhs, op, rhs } => bop_expr(dest, lhs, *op, rhs),
        Expr::Uop { op, expr } => uop_expr(dest, *op, expr),
    }
}

/// Adds a constant to an egg rec-expr.
#[must_use]
pub fn constant(dest: &mut super::Expr, k: Constant) -> Id {
    dest.add(Term::Constant(k))
}

/// Adds a PVC binary operation to an egg rec-expr.
fn bop_expr<M, V: Clone + Into<Symbol>>(
    dest: &mut super::Expr,
    lhs: &Expr<M, V>,
    op: bop::Bop,
    rhs: &Expr<M, V>,
) -> Id {
    let lhs_id = expr(dest, lhs);
    let rhs_id = expr(dest, rhs);
    dest.add((bop(op))([lhs_id, rhs_id]))
}

/// Maps from a PVC binary operation to an egg constructor.
fn bop(op: bop::Bop) -> fn([Id; 2]) -> Term {
    match op {
        bop::Bop::Arith(a) => arith_bop(a),
        bop::Bop::Bool(b) => bool_bop(b),
        bop::Bop::Rel(r) => rel_bop(r),
    }
}

/// Maps from a PVC arithmetic binary operation to an egg constructor.
fn arith_bop(op: bop::Arith) -> fn([Id; 2]) -> Term {
    match op {
        bop::Arith::Add => Term::Add,
        bop::Arith::Sub => Term::Sub,
        bop::Arith::Div => Term::Div,
        bop::Arith::Mul => Term::Mul,
        bop::Arith::Modulus => Term::Modulus,
    }
}

/// Maps from a PVC Boolean binary operation to an egg constructor.
fn bool_bop(op: bop::Bool) -> fn([Id; 2]) -> Term {
    match op {
        bop::Bool::And => Term::And,
        bop::Bool::Or => Term::Or,
        bop::Bool::Implies => Term::Implies,
        bop::Bool::Iff => Term::Iff,
    }
}

/// Maps from a PVC relational binary operation to an egg constructor.
fn rel_bop(op: bop::Rel) -> fn([Id; 2]) -> Term {
    match op {
        bop::Rel::Eq => Term::Eq,
        bop::Rel::NotEq => Term::NotEq,
        bop::Rel::Less => Term::Less,
        bop::Rel::LessEq => Term::LessEq,
        bop::Rel::Greater => Term::Greater,
        bop::Rel::GreaterEq => Term::GreaterEq,
    }
}

/// Adds a PVC unary operation to an egg rec-expr.
fn uop_expr<M, V: Clone + Into<Symbol>>(dest: &mut super::Expr, op: Uop, inner: &Expr<M, V>) -> Id {
    let expr_id = expr(dest, inner);
    dest.add((uop(op))(expr_id))
}

/// Maps from a PVC binary operation to an egg constructor.
fn uop(op: Uop) -> fn(Id) -> Term {
    match op {
        Uop::Deref => Term::Deref,
        Uop::Plus => Term::Plus,
        Uop::Minus => Term::Minus,
        Uop::Not => Term::Not,
    }
}
