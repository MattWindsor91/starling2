//! Procedure calls and related concepts.

use super::{super::tagged::Tagged, typing::Type, Identifier};

/// Generalised type of anything shaped like a procedure call, including view atoms.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Call<'inp, M, Arg> {
    pub name: Tagged<M, Identifier<'inp>>,
    pub args: Vec<Tagged<M, Arg>>,
}

/// The default call has an empty name and no arguments.
///
/// An empty name is syntactically invalid, but we assume it will be replaced with something
/// non-empty if we are emitting syntax.
impl<'inp, M: Default, Arg> Default for Call<'inp, M, Arg> {
    fn default() -> Self {
        Self {
            name: Tagged::default(),
            args: vec![],
        }
    }
}

/// Type of procedure (and non-iterated view atom) prototypes.
pub type Prototype<'inp, M, V> = Call<'inp, M, Parameter<'inp, M, V>>;

/// A formal parameter.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Parameter<'inp, M, V> {
    pub name: Identifier<'inp>,
    pub ty: Type<'inp, M, V>,
}
