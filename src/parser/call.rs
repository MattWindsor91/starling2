//! Parsers for call-related constructs.

use pest::{iterators::Pairs, Span};

use super::{
    super::language::{
        ast::{call, Identifier},
        tagged::Spanned,
    },
    expr, typing, utils, Rule,
};

//
// Prototypes
//

/// Shorthand for the type of prototype returned by `prototype`.
pub type Prototype<'inp> = call::Prototype<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Shorthand for the type of parameter returned by `parameter`.
pub type Parameter<'inp> = call::Parameter<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Parses `pairs` representing a function or view prototype.
#[must_use]
pub fn prototype(pairs: Pairs<Rule>) -> Prototype {
    utils::match_rules!(pair in pairs, proto: Prototype {
        identifier => proto.name = utils::spanned_id(&pair),
        parameter => proto.args.push(utils::lift_many(pair, parameter))
    })
}

/// Parses `pairs` representing a formal parameter.
#[must_use]
pub fn parameter(pairs: Pairs<Rule>) -> Parameter {
    utils::match_rules!(pair in pairs, param: Parameter {
        identifier => param.name = utils::spanned_id(&pair),
        starling_type => param.ty = utils::lift_one(pair, typing::starling_type)
    })
}

//
// Calls
//

/// Shorthand for the type of calls.
pub type Call<'inp> = call::Call<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Shorthand for the type of argument lists.
pub type ArgumentList<'inp> = Vec<Spanned<'inp, expr::Expr<'inp>>>;

/// Parses a call.
#[must_use]
pub fn parse(pairs: Pairs<Rule>) -> Call {
    utils::match_rules!(pair in pairs, call: Call {
        identifier => call.name = utils::spanned_id(&pair),
        argument_list => call.args = argument_list(pair.into_inner())
    })
}

/// Parses an argument list.
#[must_use]
pub fn argument_list(pairs: Pairs<Rule>) -> ArgumentList {
    utils::match_rules!(pair in pairs, args: ArgumentList {
        expr => args.push(utils::lift_many(pair, expr::parse))
    })
}
