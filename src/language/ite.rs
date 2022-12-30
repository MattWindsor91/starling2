//! The if-then-else construct and various utilities for it.

/// A generalised if-then-else.
///
/// Type `B` is the type of branches.
/// Type `C` is the type of conditions.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ite<B, C> {
    /// The branch that holds if `cond` is true.
    pub true_branch: B,
    /// The condition.
    pub cond: C,
    /// The view that holds if `cond` is false.
    pub false_branch: B,
}

impl<B, C> Ite<B, C> {
    /// Constructs a new if-then-else (using Hoare-style order of arguments).
    #[must_use]
    pub fn new(true_branch: B, cond: C, false_branch: B) -> Self {
        Self {
            true_branch,
            cond,
            false_branch,
        }
    }

    /// Attempts to evaluate an if-then-else.
    ///
    /// # Examples
    ///
    /// ```
    /// use starling::language::ite::Ite;
    ///
    /// let i = Ite::new("yes", 42, "no");
    ///
    /// assert_eq!(Some("yes"), i.eval(x -> Some(x == 42));
    /// assert_eq!(Some("no"), i.eval(x -> Some(x == 100));
    /// assert_eq!(None, i2.eval(_ -> None));
    /// ```
    #[must_use]
    pub fn eval(&self, cond_f: impl FnOnce(&C) -> Option<bool>) -> Option<&B> {
        cond_f(&self.cond).map(|b| self.branch(b))
    }

    /// Borrows the branch of the if-then-else corresponding to the given Boolean.
    ///
    /// # Examples
    ///
    /// ```
    /// use starling::language::ite::Ite;
    ///
    /// let i = Ite::new("yes", 42, "no");
    ///
    /// assert_eq!("yes", i.branch(true));
    /// assert_eq!("no", i.branch(false));
    /// ```
    #[must_use]
    pub fn branch(&self, selector: bool) -> &B {
        if selector {
            &self.true_branch
        } else {
            &self.false_branch
        }
    }
}
