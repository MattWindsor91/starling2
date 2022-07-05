//! Expressions in the high-level Starling language.

/// Constant expressions.
pub enum Const {
    /// Integer constant.
    Int(i64),
    /// Boolean constant.
    Bool(bool),
}

/// Type of expressions, parameterised over variables.
pub enum Expr<V> {
    /// Constant.
    Const(Const),
    /// Variable reference.
    Var(V),
    /// Binary (infix) operation.
    Bop {
        op: Bop,
        lhs: Box<Expr<V>>,
        rhs: Box<Expr<V>>,
    },
    /// Unary (prefix or postfix) operation.
    Uop { op: Uop, expr: Box<Expr<V>> },
}

/// Binary operators.
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

/// Unary operators.
pub enum Uop {
    /// Positive sign.
    Plus,
    /// Negative sign.
    Minus,
    /// Logical negation.
    Not,
}
