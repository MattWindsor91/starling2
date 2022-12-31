//! Parsers for call-related constructs.

use pest::{iterators::Pairs, Span};

use super::{
    super::language::{
        ast::{call, Identifier},
        tagged::Spanned,
    },
    expr, utils, Rule,
};

//
// Prototypes
//

/// Parses a `pair` representing a function prototype.
pub fn prototype(pairs: Pairs<Rule>) -> call::Prototype<Option<Span>, Identifier> {
    pairs.fold(call::Prototype::default(), |mut proto, pair| {
        match pair.as_rule() {
            Rule::identifier => proto.name = utils::spanned_id(&pair),
            r => utils::unexpected_rule(r),
        };
        proto
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
