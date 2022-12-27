//! Parser for views and view atoms.

pub mod assertion;

use pest::{iterators::Pairs, Span};

use super::{
    super::language::{
        ast::{view, Identifier},
        tagged::Spanned,
    },
    call, utils, Rule,
};

//
// Patterns
//

/// Shorthand for the type of pattern produced by the parser.
pub type Pattern<'inp> = view::Pattern<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Shorthand for the type of pattern atom produced by the parser.
pub type PatternAtom<'inp> = view::PatternAtom<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Shorthand for the type of pattern argument produced by the parser.
pub type PatternArgument<'inp> = view::PatternArgument<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Parses a view pattern given the `pairs` over its contents.
#[must_use]
pub fn pattern(pairs: Pairs<Rule>) -> Pattern {
    utils::match_rules!(pair in pairs, pat: Pattern {
        view_pattern_atom => pat.contents.push(utils::lift_many(pair, pattern_atom))
    })
}

fn pattern_atom(pairs: Pairs<Rule>) -> PatternAtom {
    utils::match_rules!(pair in pairs, pat: PatternAtom {
        identifier => pat.head.name = utils::spanned_id(pair),
        view_pattern_argument_list => pat.head.args = pattern_arguments(pair.into_inner())
    })
}

fn pattern_arguments(pairs: Pairs<Rule>) -> Vec<Spanned<PatternArgument>> {
    utils::match_rules!(pair in pairs, args: Vec<Spanned<PatternArgument>> {
        expr => args.push(utils::lift_many(pair, pattern_expr_argument))
    })
}

fn pattern_expr_argument(pairs: Pairs<Rule>) -> PatternArgument {
    PatternArgument::Expr(super::expr::parse(pairs))
}
