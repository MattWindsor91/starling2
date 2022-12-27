//! The Starling abstract syntax tree.
//!
//! This module contains declarations that capture the shape of the Starling language as parsed, and
//! are not generalisable to lowered representations.

pub use call::Generic;
pub use constraint::Constraint;
pub use decl::Decl;
pub use expr::Expr;
pub use variable::Identifier;

use super::tagged::Tagged;

pub mod call;
pub mod constraint;
pub mod decl;
pub mod expr;
pub mod stm;
pub mod typing;
pub mod variable;
pub mod view;

//
// Top-level
//

/// A program.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Program<'inp, M, V> {
    /// The name of the program.
    pub name: Tagged<M, Identifier<'inp>>,
    /// The declarations within the program.
    pub decls: Vec<Tagged<M, Decl<'inp, M, V>>>,
}

/// A default program has no declarations and a blank name.
///
/// An empty name is not syntactically valid, but we assume that users of the default program will
/// replace it.
impl<'inp, M: Default, V> Default for Program<'inp, M, V> {
    fn default() -> Self {
        Self {
            name: Tagged::default(),
            decls: vec![],
        }
    }
}
