//! The PVC control-flow graph representation.

use std::collections::HashMap;

use egg::Symbol;
use pest::Span;

use super::{typing, var};

/// A full PVC program in control-flow graph form.
///
/// This structure contains an entire
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Program<M> {
    /// Named types defined in this program.
    pub types: HashMap<Symbol, Type<M>>,
    /// Variables defined in this program.
    pub variables: HashMap<Symbol, Var<M>>,
}

/// A type record.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Type<M> {
    /// The origin of the type.
    pub origin: Origin<M>,
    /// The definition of the type.
    pub ty: typing::Type<M, Symbol>,
}

/// Information about where some semantic element originated from.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum Origin<M> {
    /// The element was synthesised.
    #[default]
    Synthesised,
    /// The element is intrinsic to the language.
    Intrinsic,
    /// The element comes from user input.
    Script(M),
}

/// A variable record.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Var<M> {
    /// The origin of the variable.
    pub origin: Origin<M>,
    /// The scope of the variable.
    pub scope: var::Scope,
    /// The type of the variable, as an index into the typing table.
    pub ty: Symbol,
}

#[derive(Debug, Clone, thiserror::Error, Eq, PartialEq)]
pub enum Error<'ast> {
    #[error("Duplicate variable")]
    DuplicateVar {
        name: &'ast str,
        first_loc: Span<'ast>,
        second_loc: Span<'ast>,
    },
}
