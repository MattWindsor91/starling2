//! Unary operators and their various impls.

use std::fmt::{Display, Formatter};

/// Unary operators.
///
/// The list of operators is subject to expansion at any time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Uop {
    /// Pointer dereference.
    Deref,
    /// Positive sign.
    Plus,
    /// Negative sign.
    Minus,
    /// Logical negation.
    Not,
}

/// Outputs a human-readable rendering of a unary operator.
impl Display for Uop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Deref => "^",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Not => "not",
        })
    }
}

impl Uop {
    /// Gets the fixity of an operator.
    #[must_use]
    pub fn fixity(self) -> Fixity {
        match self {
            Self::Deref => Fixity::Postfix,
            _ => Fixity::Prefix,
        }
    }
}

/// Fixity of a unary operator.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Fixity {
    Prefix,
    Postfix,
}
