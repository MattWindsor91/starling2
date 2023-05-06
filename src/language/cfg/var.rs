//! The control-flow graph representation of variables.
//!
//! Variables form a global symbol table resolved in one of the early binding passes.
use super::{BlockRef, Origin};
use std::collections::HashMap;

/// A variable map.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Map<M> {
    /// The variable mapping.
    contents: HashMap<Symbol, Record<M>>,
}

impl<M> Map<M> {
    /// Resolves `sym` to a variable.
    ///
    /// If `sym` cannot be found in the requested block, we recursively ascend in scope until we
    /// either find it or we reach the global variables and still fail to find it.
    pub fn get(&self, sym: &Symbol) -> Option<(Symbol, &Record<M>)> {
        let mut sym = Some(sym.clone());

        while let Some(s) = sym {
            if let Some(v) = self.contents.get(&s) {
                return Some((s, v));
            }

            sym = s.parent();
        }

        None
    }
}

/// A variable symbol.
///
/// Because variables can be declared at various blocks in the program, and
/// starling2 is lexically scoped, we track references to the block in which
/// a variable has been declared.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Symbol {
    /// The block in which the variable has been declared.
    pub block: BlockRef,
    /// The name of the variable.
    pub name: egg::Symbol,
}

impl Symbol {
    /// Gets a symbol with the same name as this one, but at the parent block level (if any).
    pub fn parent(&self) -> Option<Symbol> {
        self.block.parent().map(|block| Self {
            block,
            name: self.name,
        })
    }
}

/// A variable record.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Record<M> {
    /// The origin of the variable.
    pub origin: Origin<M>,
    /// The scope of the variable (thread-local or shared).
    ///
    /// It is ill-formed for local variables to be shared.
    pub scope: super::super::var::Scope,
    /// The type of the variable, as an index into the typing table.
    pub ty: egg::Symbol,
}
