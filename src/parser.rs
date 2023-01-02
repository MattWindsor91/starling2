//! Top-level of the PVC ('Pascal with Views and Concurrency') parser.
//!
//! The PVC parser is based on `pest`, and has two parts.  First, we use the Pest grammar
//! (`starling.pest`) to parse PVC scripts into Pest pairs.  Then, we use the recursive descent
//! functions within the submodules of this module to massage those pairs into AST nodes (this stage
//! cannot produce further parse errors so long as the Pest grammar is well-formed).

use pest::Parser;

use super::language::{ast, tagged::Spanned};

mod call;
mod constraint;
mod expr;
mod program;
mod stm;
mod typing;
mod utils;
mod var;
mod view;

/// The PVC parser.
#[derive(pest_derive::Parser)]
#[grammar = "parser/pvc.pest"]
struct Pvc;

/// Parses a program.
///
/// # Errors
///
/// Fails if `input` could not be parsed correctly.
pub fn parse(input: &str) -> Result<Spanned<program::Program>> {
    let pairs = Pvc::parse(Rule::program, input).map_err(Box::new)?;
    let pair = utils::one(pairs);
    Ok(utils::lift_many(pair, program::parse))
}

/// Errors returned by the Starling parser.
#[derive(Debug, Clone, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Parser error: {0}")]
    Parse(#[from] Box<pest::error::Error<Rule>>),
}

/// Shorthand for a result over [Error].
pub type Result<T> = std::result::Result<T, Error>;
