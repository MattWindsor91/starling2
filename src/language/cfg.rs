//! The PVC control-flow graph representation.

use std::collections::HashMap;

use egg::Symbol;
use pest::Span;

use super::typing;

mod var;

/// A full PVC program in control-flow graph form.
///
/// This structure contains an entire program as a series of symbol tables.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Program<M> {
    /// Named types defined in this program.
    pub types: HashMap<Symbol, Type<M>>,
    /// Variables defined in this program.
    pub variables: var::Map<M>,
}

/// An unambiguous path to a particular block in a program.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum BlockRef {
    /// Not in a block, but rather in the global scope.
    Global,
    /// Local to the procedure with the given name, and indirect through the given block path.
    Proc(Symbol, Vec<usize>),
}

impl BlockRef {
    /// Gets the parent scope of the
    pub fn parent(&self) -> Option<BlockRef> {
        match self {
            Self::Global => None,
            Self::Proc(proc, blocks) => Some(proc_parent(proc, blocks)),
        }
    }
}

fn proc_parent(proc: &Symbol, blocks: &[usize]) -> BlockRef {
    let mut blocks = blocks.to_vec();
    if blocks.pop().is_some() {
        BlockRef::Proc(*proc, blocks)
    } else {
        BlockRef::Global
    }
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

#[derive(Debug, Clone, thiserror::Error, Eq, PartialEq)]
pub enum Error<'ast> {
    #[error("Duplicate variable")]
    DuplicateVar {
        name: &'ast str,
        first_loc: Span<'ast>,
        second_loc: Span<'ast>,
    },
}
