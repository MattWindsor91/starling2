//! Top-level of the Starling parser.

use itertools::Itertools;
use pest::{iterators::Pairs, Parser, Span};

use super::language::{
    ast::{Identifier, Program},
    tagged::Spanned,
};

mod decl;
mod utils;

/// The Starling parser.
#[derive(pest_derive::Parser)]
#[grammar = "parser/starling.pest"]
struct StarlingParser;

/// Parses a program.
pub fn parse(input: &str) -> Result<Spanned<Program<Span, Identifier>>> {
    let pairs = StarlingParser::parse(Rule::program, input).map_err(Box::new)?;
    let pair = pairs
        .exactly_one()
        .expect("parser should only allow one program here");

    let span = pair.as_span();

    let program = program(pair.into_inner());

    Ok(Spanned::new(span, program))
}

fn program(body_pairs: Pairs<Rule>) -> Program<Span, Identifier> {
    body_pairs.fold(Program::default(), |mut program, pair| {
        match pair.as_rule() {
            Rule::identifier => program.name = Identifier::from(pair.as_str()),
            Rule::decl => program.declarations.push(decl::spanned(pair)),
            Rule::EOI => (),
            r => utils::unexpected_rule(r),
        };
        program
    })
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
