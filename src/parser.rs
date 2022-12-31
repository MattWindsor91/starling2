//! Top-level of the Starling parser.

use pest::Parser;

use super::language::{ast, tagged::Spanned};

mod call;
mod constraint;
mod expr;
mod program;
mod stm;
mod typing;
mod utils;
mod view;

/// The Starling parser.
#[derive(pest_derive::Parser)]
#[grammar = "parser/starling.pest"]
struct StarlingParser;

/// Parses a program.
///
/// # Errors
///
/// Fails if `input` could not be parsed correctly.
pub fn parse(input: &str) -> Result<Spanned<program::Program>> {
    let pairs = StarlingParser::parse(Rule::program, input).map_err(Box::new)?;
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
