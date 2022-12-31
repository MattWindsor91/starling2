//! Parser for view patterns.

use pest::{iterators::Pairs, Span};

use super::super::{
    super::language::{
        ast::{view::pattern, Identifier},
        tagged::Spanned,
    },
    expr, utils, Rule,
};

/// Shorthand for the type of pattern produced by the parser.
pub type Pattern<'inp> = pattern::Pattern<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Shorthand for the type of pattern atom produced by the parser.
pub type Atom<'inp> = pattern::Atom<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Shorthand for the type of pattern argument produced by the parser.
pub type Argument<'inp> = pattern::Argument<'inp, Option<Span<'inp>>, Identifier<'inp>>;

/// Parses a view pattern given the `pairs` over its contents.
#[must_use]
pub fn parse(pairs: Pairs<Rule>) -> Pattern {
    utils::match_rules!(pair in pairs, pat: Pattern {
        view_pattern_atom => pat.contents.push(utils::lift_many(pair, atom))
    })
}

fn atom(pairs: Pairs<Rule>) -> Atom {
    utils::match_rules!(pair in pairs, pat: Atom {
        identifier => pat.name = utils::spanned_id(&pair),
        view_pattern_argument_list => pat.args = arguments(pair.into_inner()),
        iterator_pattern => pat.iterator = utils::lift_many(pair, argument)
    })
}

fn arguments(pairs: Pairs<Rule>) -> Vec<Spanned<Argument>> {
    utils::match_rules!(pair in pairs, args: Vec<Spanned<Argument >> {
        expr => args.push(utils::lift_many(pair, argument))
    })
}

fn argument(pairs: Pairs<Rule>) -> Argument {
    Argument::Expr(expr::parse(pairs))
}
