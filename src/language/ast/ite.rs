//! If-then-else over AST expressions.

use super::{
    super::{ite, tagged},
    Expr,
};

/// Type of if-then-else constructs over AST expressions.
pub type Ite<'inp, M, V, B> = ite::Ite<B, tagged::Tagged<M, Expr<'inp, M, V>>>;
