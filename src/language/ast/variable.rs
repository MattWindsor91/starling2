//! Variables in Starling.
//!
//! When first parsed, variables are just string [Identifier]s.  Further semantic analysis performs
//! binding, marking, and other such augmentation.

/// A string identifier.
pub type Identifier<'inp> = std::borrow::Cow<'inp, str>;
