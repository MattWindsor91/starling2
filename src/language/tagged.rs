//! Constructs for tagging AST nodes and other language elements with metadata.

use std::fmt::{Display, Formatter};

/// A pair of AST node and its metadata.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Tagged<M, T> {
    pub meta: M,
    pub item: T,
}

impl<M, T> Tagged<M, T> {
    /// Constructs a new tagged node.
    #[must_use]
    pub fn new(meta: M, item: T) -> Self {
        Self { meta, item }
    }
}

/// Tagged items are displayed by ignoring the metadata.
impl<M, T: Display> Display for Tagged<M, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.item.fmt(f)
    }
}

/// An AST node boxed and tagged with metadata.
pub type Box<M, T> = Tagged<M, std::boxed::Box<T>>;

/// An AST node tagged with an optional Pest span.
pub type Spanned<'inp, T> = Tagged<Option<pest::Span<'inp>>, T>;

/// An AST node boxed and tagged with a Pest span.
pub type SpannedBox<'inp, T> = Spanned<'inp, std::boxed::Box<T>>;
