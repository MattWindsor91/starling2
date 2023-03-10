//! AST nodes for variables.

use super::{
    super::{
        tagged::Tagged,
        var::{Scope, Variable},
    },
    call::Parameter,
};
use egg::Symbol;

/// A variable declaration.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Decl<'inp, M, V> {
    /// Scope of the declaration.
    pub scope: Tagged<M, Scope>,
    /// Variables in the declaration, modelled as parameters.
    pub vars: Vec<Tagged<M, Parameter<'inp, M, V>>>,
}

/// The default decl is an empty shared-scope decl.
///
/// We hand-roll this implementation to avoid unnecessary bounds on `V`.
impl<'inp, M: Default, V> Default for Decl<'inp, M, V> {
    fn default() -> Self {
        Self {
            scope: Tagged::default(),
            vars: vec![],
        }
    }
}

/// A string identifier.
pub type Identifier<'inp> = std::borrow::Cow<'inp, str>;

/// Identifiers are variable references.
impl<'inp> Variable for Identifier<'inp> {
    fn to_symbol(&self) -> Symbol {
        Symbol::new(self)
    }

    fn into_symbol(self) -> Symbol {
        Symbol::new(self)
    }
}
