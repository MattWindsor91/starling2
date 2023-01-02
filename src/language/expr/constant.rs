//! Literal expressions and their various impls.

use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use num_bigint::BigInt;
use num_traits::Zero;

use super::super::typing;

/// Literal expressions.
///
/// Note that the ordering on constants is overly permissive, and orders constants across types when
/// they should not be.  We implement the ordering as such to facilitate using constants within
/// `egg` expressions.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Constant {
    /// Boolean constant.
    Bool(bool),
    /// Integer constant.
    Int(BigInt),
}

/// Type of constant conversion errors.
///
/// All constant conversion errors are typing errors, but over primitive types, and so they have
/// neither metadata nor variables.
pub type ConvertError = typing::Error<(), ()>;

/// We can convert Booleans to constants.
impl From<bool> for Constant {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

/// We can convert integers to constants.
impl From<BigInt> for Constant {
    fn from(value: BigInt) -> Self {
        Self::Int(value)
    }
}

impl TryFrom<Constant> for bool {
    type Error = ConvertError;

    fn try_from(value: Constant) -> Result<Self, Self::Error> {
        value
            .as_bool()
            .ok_or_else(|| convert_error(typing::Prim::Bool, &value))
    }
}

impl TryFrom<Constant> for BigInt {
    type Error = ConvertError;

    fn try_from(value: Constant) -> Result<Self, Self::Error> {
        value
            .as_int()
            .cloned()
            .ok_or_else(|| convert_error(typing::Prim::Int, &value))
    }
}

fn convert_error(got: typing::Prim, value: &Constant) -> ConvertError {
    ConvertError::Mismatch {
        got: typing::Type::Prim(got),
        want: typing::Type::Prim(value.prim_type()),
    }
}

impl Constant {
    /// Constructs the constant zero.
    ///
    /// We can't implement `num_traits::Zero` for [Constant], because it requires an additive
    /// identity.
    #[must_use]
    pub fn zero() -> Self {
        BigInt::zero().into()
    }

    /// Checks whether this literal is zero.
    ///
    /// We can't implement `num_traits::Zero` for [Constant], because it requires an additive
    /// identity.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        if let Self::Int(i) = self {
            i.is_zero()
        } else {
            false
        }
    }

    /// Tries to borrow the constant as a Boolean.
    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        if let Self::Bool(x) = self {
            Some(*x)
        } else {
            None
        }
    }

    /// Tries to borrow the literal as an integer.
    #[must_use]
    pub const fn as_int(&self) -> Option<&BigInt> {
        if let Self::Int(x) = self {
            Some(x)
        } else {
            None
        }
    }

    /// Gets the primitive type of this constant.
    #[must_use]
    pub const fn prim_type(&self) -> typing::Prim {
        match self {
            Self::Bool(_) => typing::Prim::Bool,
            Self::Int(_) => typing::Prim::Int,
        }
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i) => i.fmt(f),
            Self::Bool(b) => b.fmt(f),
        }
    }
}

/// We currently parse any literal that is not a Boolean as an integer.
impl FromStr for Constant {
    type Err = num_bigint::ParseBigIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.eq_ignore_ascii_case("true") {
            Self::Bool(true)
        } else if s.eq_ignore_ascii_case("false") {
            Self::Bool(false)
        } else {
            Self::Int(s.parse()?)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tries parsing true and false, while checking case-sensitivity.
    #[test]
    fn parse_booleans() {
        assert_eq!(Constant::Bool(true), "True".parse().unwrap());
        assert_eq!(Constant::Bool(false), "false".parse().unwrap());
    }
}
