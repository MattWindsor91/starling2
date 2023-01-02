//! The if-then-else construct and various utilities for it.

use super::tagged::Tagged;

/// A generalised if-then-else.
///
/// Type `M` is the type of metadata.
/// Type `B` is the type of branches.
/// Type `C` is the type of conditions.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ite<M, B, C> {
    /// The branch that holds if `cond` is true.
    pub true_branch: B,
    /// The condition.
    pub cond: Tagged<M, Condition<C>>,
    /// The branch that holds if `cond` is false.
    pub false_branch: B,
}

impl<M: Default, B: Default, C> Default for Ite<M, B, C> {
    fn default() -> Self {
        Self::nondeterministic(B::default(), B::default())
    }
}

impl<M: Default, B, C> Ite<M, B, C> {
    /// Constructs a new non-deterministic if-then-else.
    #[must_use]
    pub fn nondeterministic(true_branch: B, false_branch: B) -> Self {
        Self::new(true_branch, Condition::Nondeterministic, false_branch)
    }

    /// Constructs a new deterministic if-then-else (using Hoare-style order of arguments).
    #[must_use]
    pub fn deterministic(true_branch: B, cond: impl Into<Tagged<M, C>>, false_branch: B) -> Self {
        Self::new(
            true_branch,
            cond.into().map(Condition::Deterministic),
            false_branch,
        )
    }

    /// Constructs a new if-then-else (using Hoare-style order of arguments).
    #[must_use]
    pub fn new(true_branch: B, cond: impl Into<Tagged<M, Condition<C>>>, false_branch: B) -> Self {
        Self {
            true_branch,
            cond: cond.into(),
            false_branch,
        }
    }

    /// Attempts to evaluate an if-then-else deterministically.
    ///
    /// The evaluation takes as an argument a function for evaluating a condition to a Boolean;
    /// this function may be partial (ie, there are some conditions for which we cannot guarantee
    /// a statically known truth value),
    ///
    /// # Examples
    ///
    /// For a deterministic if-then-else, we can always evaluate to one of the branches provided
    /// that `cond_f` returns a truth value:
    ///
    /// ```
    /// use starling::language::{tagged::Tagged, ite::Ite};
    ///
    /// let i: Ite<(), _, _> = Ite::deterministic("yes", 42, "no");
    ///
    /// assert_eq!(Some(&"yes"), i.eval(|x| Some(*x == 42)));
    /// assert_eq!(Some(&"no"), i.eval(|x| Some(*x == 100)));
    /// assert_eq!(None, i.eval(|_| None));
    /// ```
    ///
    /// Nondeterministic conditions never evaluate:
    ///
    /// ```
    /// use starling::language::ite::Ite;
    ///
    /// let i: Ite<(), _, i64> = Ite::nondeterministic("yes", "no");
    ///
    /// assert_eq!(None, i.eval(|x| Some(*x == 42)));
    /// assert_eq!(None, i.eval(|x| Some(*x == 100)));
    /// assert_eq!(None, i.eval(|_| None));
    /// ```
    #[must_use]
    pub fn eval(&self, cond_f: impl FnOnce(&C) -> Option<bool>) -> Option<&B> {
        self.cond
            .item
            .as_det()
            .and_then(cond_f)
            .map(|b| self.branch(b))
    }

    /// Borrows the branch of the if-then-else corresponding to the given Boolean.
    ///
    /// # Examples
    ///
    /// ```
    /// use starling::language::ite::Ite;
    ///
    /// let i : Ite<(), _, _> = Ite::deterministic("yes", 42, "no");
    ///
    /// assert_eq!(&"yes", i.branch(true));
    /// assert_eq!(&"no", i.branch(false));
    /// ```
    #[must_use]
    pub fn branch(&self, selector: bool) -> &B {
        if selector {
            &self.true_branch
        } else {
            &self.false_branch
        }
    }

    /// Mutably borrows the branch of the if-then-else corresponding to the given Boolean.
    ///
    /// # Examples
    ///
    /// ```
    /// use starling::language::ite::Ite;
    ///
    /// let mut i : Ite<(), _, _> = Ite::deterministic("yes", 42, "no");
    /// *i.branch_mut(true) = "ja";
    ///
    /// assert_eq!(&"ja", i.branch(true));
    /// assert_eq!(&"no", i.branch(false));
    /// ```
    #[must_use]
    pub fn branch_mut(&mut self, selector: bool) -> &mut B {
        if selector {
            &mut self.true_branch
        } else {
            &mut self.false_branch
        }
    }
}

/// A condition in an AST if-then-else.
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum Condition<C> {
    /// A nondeterministic condition.
    #[default]
    Nondeterministic,
    /// A deterministic condition.
    Deterministic(C),
}

impl<C> Condition<C> {
    /// Borrows the underlying condition if this condition is deterministic.
    #[must_use]
    pub fn as_det(&self) -> Option<&C> {
        if let Self::Deterministic(c) = self {
            Some(c)
        } else {
            None
        }
    }
}
