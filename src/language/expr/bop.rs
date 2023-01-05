//! Unary operators and their various impls.

use std::fmt::{Display, Formatter};

/// Binary operators.
///
/// The list of operators is subject to expansion at any time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Bop {
    /// Arithmetic operator.
    Arith(Arith),
    /// Boolean operator.
    Bool(Bool),
    /// Relational operator.
    Rel(Rel),
}

/// Outputs a human-readable rendering of a binary operator.
impl Display for Bop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Bop::Arith(a) => a.fmt(f),
            Bop::Bool(b) => b.fmt(f),
            Bop::Rel(r) => r.fmt(f),
        }
    }
}

/// Arithmetic binary operators.
///
/// The list of operators is subject to expansion at any time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Arith {
    /// Addition.
    Add,
    /// Subtraction.
    Sub,
    /// Integer division.
    Div,
    /// Multiplication.
    Mul,
    /// Integer modulus.
    Modulus,
}

/// We can lift arithmetic binary operators into binary operators.
impl From<Arith> for Bop {
    fn from(value: Arith) -> Self {
        Self::Arith(value)
    }
}

/// Outputs a human-readable rendering of a binary arithmetic operator.
impl Display for Arith {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Arith::Add => "+",
            Arith::Sub => "-",
            Arith::Mul => "*",
            Arith::Div => "div",
            Arith::Modulus => "mod",
        })
    }
}

/// Binary Boolean operators.
///
/// The list of operators is subject to expansion at any time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Bool {
    /// Logical AND.
    And,
    /// Logical OR.
    Or,
    /// Classical logical implication (not-LHS-or-RHS).
    Implies,
    /// If and only if.
    Iff,
}

/// We can lift arithmetic binary operators into binary operators.
impl From<Bool> for Bop {
    fn from(value: Bool) -> Self {
        Self::Bool(value)
    }
}

/// Outputs a human-readable rendering of a binary Boolean operator.
impl Display for Bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Bool::And => "and",
            Bool::Or => "or",
            Bool::Implies => "implies",
            Bool::Iff => "iff",
        })
    }
}

/// Binary relational operators.
///
/// The list of operators is subject to expansion at any time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Rel {
    /// Equal to.
    Eq,
    /// Not equal to.
    NotEq,
    /// Less than.
    Less,
    /// Less than or equal to.
    LessEq,
    /// Greater than.
    Greater,
    /// Greater than or equal to.
    GreaterEq,
}

/// We can lift relational binary operators into binary operators.
impl From<Rel> for Bop {
    fn from(value: Rel) -> Self {
        Self::Rel(value)
    }
}

/// Outputs a human-readable rendering of a binary relational operator.
impl Display for Rel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Rel::Eq => "=",
            Rel::NotEq => "<>",
            Rel::Less => "<",
            Rel::LessEq => "<=",
            Rel::Greater => ">",
            Rel::GreaterEq => ">=",
        })
    }
}
