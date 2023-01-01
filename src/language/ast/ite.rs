//! If-then-else over AST expressions.

use super::super::{expr::Expr, ite};

/// Type of if-then-else constructs over AST expressions.
pub type Ite<M, V, B> = ite::Ite<M, B, Expr<M, V>>;

/// Type of if-then-else conditionals.
pub type Condition<M, V> = ite::Condition<Expr<M, V>>;
