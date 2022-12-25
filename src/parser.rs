//! Top-level of the Starling parser.

/// The Starling parser.
#[derive(pest_derive::Parser)]
#[grammar = "parser/starling.pest"]
pub struct Parser;
