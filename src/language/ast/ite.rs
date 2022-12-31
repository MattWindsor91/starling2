//! If-then-else over AST expressions.

use super::{super::ite, Expr};

/// Type of if-then-else constructs over AST expressions.
pub type Ite<'inp, M, V, B> = ite::Ite<M, B, Expr<'inp, M, V>>;

/// Type of if-then-else conditionals.
pub type Condition<'inp, M, V> = ite::Condition<Expr<'inp, M, V>>;
