//! Procedure calls and related concepts.

use super::Identifier;
use crate::language::ast::typing::Type;

/// Generalised type of anything shaped like a procedure call, including view atoms.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Call<'inp, Arg> {
    pub name: Identifier<'inp>,
    pub args: Vec<Arg>,
}

/// Type of procedure (and non-iterated view atom) prototypes.
pub type Prototype<'inp, M, V> = Call<'inp, Parameter<'inp, M, V>>;

/// A formal parameter.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Parameter<'inp, M, V> {
    pub name: Identifier<'inp>,
    pub ty: Type<M, V>,
}
