//! Utilities for the parser.

use itertools::Itertools;
use pest::iterators::{Pair, Pairs};

/// Enforces that exactly one pair exists in `pairs`, and extracts it.
///
/// # Panics
///
/// Panics if there is more or less than one pair in `pairs`.
pub fn one(pairs: Pairs<super::Rule>) -> Pair<super::Rule> {
    pairs
        .exactly_one()
        .expect("expected exactly one match here")
}

/// Panics with an unexpected rule.
///
/// # Panics
///
/// Always.
pub fn unexpected_rule(rule: super::Rule) -> ! {
    unreachable!("parser should not have accepted {:?} here", rule)
}
