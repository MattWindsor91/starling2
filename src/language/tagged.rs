//! Constructs for tagging AST nodes and other language elements with metadata.

use std::fmt::{Display, Formatter};

/// A pair of AST node and its metadata.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Tagged<M, T> {
    /// Metadata attached to the item.
    pub meta: M,
    /// Item being tagged.
    pub item: T,
}

impl<M, T> Tagged<M, T> {
    /// Constructs a new tagged node.
    #[must_use]
    pub fn new(meta: M, item: T) -> Self {
        Self { meta, item }
    }

    /// Maps a function over the item of a tagged node.
    #[must_use]
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Tagged<M, U> {
        Tagged::new(self.meta, f(self.item))
    }

    /// Tries to map a function over the item of a tagged node.
    ///
    /// # Errors
    ///
    /// Fails if `f` fails to map over the item.
    pub fn try_map<U, E>(self, f: impl FnOnce(T) -> Result<U, E>) -> Result<Tagged<M, U>, E> {
        Ok(Tagged::new(self.meta, f(self.item)?))
    }

    /// Maps a function `f` over the direct metadata of this tagged node.
    ///
    /// See `map_meta` in the `HasMeta` trait for recursive mapping.
    pub fn map_direct_meta<N>(self, f: impl FnOnce(M) -> N) -> Tagged<N, T> {
        Tagged::new(f(self.meta), self.item)
    }

    /// Maps a possibly error-prone function `f` over the direct metadata of this tagged node.
    ///
    /// # Errors
    ///
    /// Fails if `f` fails on the metadata.
    pub fn try_map_direct_meta<N, E>(
        self,
        f: impl FnOnce(M) -> Result<N, E>,
    ) -> Result<Tagged<N, T>, E> {
        Ok(Tagged::new(f(self.meta)?, self.item))
    }
}

impl<M: Default, T> Tagged<M, T> {
    /// Constructs a new tagged node with default metadata.
    #[must_use]
    pub fn with_default(item: T) -> Self {
        Self::new(M::default(), item)
    }
}

/// We can convert untagged things into tagged things so long as there is a default tag.
impl<M: Default, T> From<T> for Tagged<M, T> {
    fn from(value: T) -> Self {
        Self::with_default(value)
    }
}

/// Tagged items are displayed by ignoring the metadata.
impl<M, T: Display> Display for Tagged<M, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.item.fmt(f)
    }
}

/// An AST node tagged with an optional Pest span.
pub type Spanned<'inp, T> = Tagged<Option<pest::Span<'inp>>, T>;
