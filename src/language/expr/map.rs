//! Mapping and traversing over expressions.

use std::convert::Infallible;

use super::super::{tagged::Tagged, var::Variable};

/// Mapping of functions over variables in expressions.
pub trait HasVars<V>: Sized {
    /// The output type of the mapping process.
    type Output<U>;

    /// Maps a function `f` over the variables of part of an expression.
    fn map_var<U>(self, mut f: impl FnMut(V) -> U) -> Self::Output<U> {
        self.try_map_var(|x| Ok::<U, Infallible>(f(x)))
            .expect("this can't possibly error")
    }

    /// Maps a possibly error-prone function `f` over the variables of part of an expression.
    ///
    /// # Errors
    ///
    /// Fails if `f` fails on any variable.
    fn try_map_var<U, E>(self, f: impl FnMut(V) -> Result<U, E>) -> Result<Self::Output<U>, E>;
}

/// We can trivially map over the variables of a single variable.
impl<V: Variable> HasVars<V> for V {
    type Output<U> = U;

    fn map_var<U>(self, mut f: impl FnMut(V) -> U) -> Self::Output<U> {
        f(self)
    }

    fn try_map_var<U, E>(self, mut f: impl FnMut(V) -> Result<U, E>) -> Result<Self::Output<U>, E> {
        f(self)
    }
}

/// We can map over the variables of a tagged node if its items can be mapped over.
impl<M, V, T: HasVars<V>> HasVars<V> for Tagged<M, T> {
    type Output<U> = Tagged<M, T::Output<U>>;

    fn map_var<U>(self, f: impl FnMut(V) -> U) -> Self::Output<U> {
        self.map(|x| x.map_var(f))
    }

    fn try_map_var<U, E>(self, f: impl FnMut(V) -> Result<U, E>) -> Result<Self::Output<U>, E> {
        self.try_map(|x| x.try_map_var(f))
    }
}

//
// Metadata
//

/// Mapping of functions over metadata in expressions.
pub trait HasMeta<M>: Sized {
    /// The output type of the mapping process.
    type Output<N>;

    /// Maps a function `f` over the metadata of part of an expression.
    fn map_meta<N>(self, mut f: impl FnMut(M) -> N) -> Self::Output<N> {
        self.try_map_meta(|x| Ok::<N, Infallible>(f(x)))
            .expect("this can't possibly error")
    }

    /// Maps a possibly error-prone function `f` over the metadata of part of an expression.
    ///
    /// # Errors
    ///
    /// Fails if `f` fails on any metadata.
    fn try_map_meta<N, E>(self, f: impl FnMut(M) -> Result<N, E>) -> Result<Self::Output<N>, E>;
}

/// We can map over the metadata of a tagged node.
impl<M, T: HasMeta<M>> HasMeta<M> for Tagged<M, T> {
    type Output<N> = Tagged<N, T::Output<N>>;

    fn map_meta<N>(self, mut f: impl FnMut(M) -> N) -> Self::Output<N> {
        Tagged::new(f(self.meta), self.item.map_meta(f))
    }

    fn try_map_meta<N, E>(
        self,
        mut f: impl FnMut(M) -> Result<N, E>,
    ) -> Result<Self::Output<N>, E> {
        Ok(Tagged::new(f(self.meta)?, self.item.try_map_meta(f)?))
    }
}
