//! Top-level of the Starling parser.

use pest::{iterators::Pairs, Parser, Span};

use super::language::{ast, tagged::Spanned};

mod call;
mod constraint;
mod decl;
mod expr;
mod stm;
mod typing;
mod utils;
mod view;

/// The Starling parser.
#[derive(pest_derive::Parser)]
#[grammar = "parser/starling.pest"]
struct StarlingParser;

/// Type of program as parsed by the parser.
pub type Program<'inp> = ast::Program<'inp, Option<Span<'inp>>, ast::Identifier<'inp>>;

/// Parses a program.
///
/// # Errors
///
/// Fails if `input` could not be parsed correctly.
pub fn parse(input: &str) -> Result<Spanned<Program>> {
    let pairs = StarlingParser::parse(Rule::program, input).map_err(Box::new)?;
    let pair = utils::one(pairs);

    let span = pair.as_span();

    let program = program(pair.into_inner());

    Ok(Spanned::new(Some(span), program))
}

fn program(pairs: Pairs<Rule>) -> Program {
    utils::match_rules!(pair in pairs, prog: Program {
        identifier => prog.name = utils::spanned_id(&pair),
        decl => prog.decls.push(utils::lift_one(pair, decl::parse)),
        EOI => ()
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
