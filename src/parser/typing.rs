//! Parsers for types.

use pest::{iterators::Pair, Span};

use super::{
    super::language::{ast::Identifier, typing},
    utils, Rule,
};

/// Shorthand for the type of type parsed by `starling_type`.
pub type Type<'inp> = typing::Type<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Parses `pair` as a Starling type.
#[must_use]
pub fn starling_type(pair: Pair<Rule>) -> Type {
    utils::match_rule!(pair {
        primitive_type => Type::Prim(primitive_type(&utils::one_inner(pair)))
        // TODO(@MattWindsor91): other forms of type
    })
}

fn primitive_type(pair: &Pair<Rule>) -> typing::Prim {
    utils::match_rule!(pair {
        integer_type => typing::Prim::Int,
        boolean_type => typing::Prim::Bool
    })
}
