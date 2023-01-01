//! Language constructs for Starling.
//!
//! This module contains several components:
//!
//! - the _abstract syntax tree_, which is the representation used for PVC proof scripts on
//!   parsing;
//! - the _control flow graph_, which is the intermediate representation used for PVC proof
//!   scripts during lowering and emission;
//! - the Starling _type system_;
//! - metadata tagging and other utilities.

pub mod ast;
pub mod cfg;
pub mod expr;
pub mod ite;
pub mod tagged;
pub mod typing;
pub mod var;
