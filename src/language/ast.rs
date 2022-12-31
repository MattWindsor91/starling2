//! The Starling abstract syntax tree.
//!
//! This module contains declarations that capture the shape of the Starling language as parsed, and
//! are not generalisable to lowered representations.

// Convenience exports for the most common and unambiguous pieces of AST.
pub use call::Call;
pub use constraint::Constraint;
pub use expr::Expr;
pub use program::{Decl, Procedure, Program};
pub use stm::{Stm, Triple};
pub use typing::Type;
pub use variable::Identifier;

pub mod call;
pub mod constraint;
pub mod expr;
pub mod ite;
pub mod program;
pub mod stm;
pub mod typing;
pub mod variable;
pub mod view;
