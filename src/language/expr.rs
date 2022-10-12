//! Expressions in the high-level Starling language.

pub mod bop;
pub mod literal;
pub mod uop;

pub use bop::Bop;
pub use literal::Literal;
use std::fmt::{Display, Formatter};
pub use uop::Uop;

/// Type of expressions, parameterised over tags and variables.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Expr<T, V> {
    /// Tag attached to an expression, containing metadata.
    pub tag: T,
    /// The body of the expression.
    pub body: Body<T, V>,
}

impl<V> Expr<(), V> {
    /// Constructs a tagless expression by lifting a raw body.
    pub fn new(body: Body<(), V>) -> Self {
        Self { tag: (), body }
    }
}

/// We can display expressions, hiding the tags, by delegating to the bodies.
impl<T, V: Display> Display for Expr<T, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.body.fmt(f)
    }
}

/// The body of an expression, parameterised over tags and variables.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Body<T, V> {
    /// Literal expression.
    Literal(Literal),
    /// Variable reference.
    Var(V),
    /// Binary (infix) operation.
    Bop {
        op: Bop,
        lhs: Box<Expr<T, V>>,
        rhs: Box<Expr<T, V>>,
    },
    /// Unary (prefix or postfix) operation.
    Uop { op: Uop, expr: Box<Expr<T, V>> },
}

impl<T, V: Display> Display for Body<T, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(l) => l.fmt(f),
            Self::Var(v) => v.fmt(f),
            // TODO: precedence
            Self::Bop { op, lhs, rhs } => write!(f, "({lhs}) {op} ({rhs})"),
            Self::Uop { op, expr } => write!(f, "{op}({expr})"),
        }
    }
}
