//! Unary operators and their various impls.

use std::fmt::{Display, Formatter};

/// Binary operators.
///
/// The list of operators is subject to expansion at any time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Bop {
    /// Addition.
    Add,
    /// Subtraction.
    Sub,
    /// Multiplication.
    Mul,
    /// Integer division.
    Div,
    /// Integer modulus.
    Mod,
    /// Logical conjunction.
    And,
    /// Logical disjunction.
    Or,
    /// Classical logical implication (not-LHS-or-RHS).
    Impl,
    /// If and only if.
    Iff,
}

/// Outputs a human-readable rendering of a binary operator.
impl Display for Bop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Bop::Add => "+",
            Bop::Sub => "-",
            Bop::Mul => "*",
            Bop::Div => "/",
            Bop::Mod => "%",
            Bop::And => "and",
            Bop::Or => "or",
            Bop::Impl => "implies",
            Bop::Iff => "iff",
        })
    }
}
