//! The starling(2) language, parser, semantics, and related items.
//!
//! This library contains:
//!
//! - the Starling language in its various forms (AST, CFG, and so on);
//! - the Starling parser;
//! - Starling semantic analysis and lowering;
//! - Verification condition generation in various output formats.

#![warn(clippy::pedantic)]

pub mod binder;
pub mod language;
pub mod parser;
