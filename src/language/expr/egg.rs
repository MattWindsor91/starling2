//! `egg` language for Starling expressions.

use egg::{Id, Symbol};

egg::define_language! {
    enum Expr {
        // Arithmetic binary operations
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "/" = Div([Id; 2]),
        "*" = Mul([Id; 2]),
        "div" = IntDiv([Id; 2]),
        "mod" = Modulus([Id; 2]),
        // Boolean binary operations
        "and" = And([Id; 2]),
        "or" = Or([Id; 2]),
        "implies" = Implies([Id; 2]),
        "iff" = Iff([Id; 2]),
        // Relational binary operations
        "<" = Less([Id; 2]),
        "<=" = LessEq([Id; 2]),
        "=" = Eq([Id; 2]),
        "<>" = NotEq([Id; 2]),
        ">=" = GreaterEq([Id; 2]),
        ">" = Greater([Id; 2]),
        // Prefix operations
        "-" = Minus(Id),
        "+" = Plus(Id),
        // Postfix operations
        "^" = Deref(Id),
        // Literals
        Int(num_bigint::BigInt),
        Bool(bool),
        Symbol(Symbol),
    }
}
