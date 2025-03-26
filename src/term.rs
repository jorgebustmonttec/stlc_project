mod display;
pub mod parse;
pub mod util;

use std::collections::HashSet;

/// Represents a lambda calculus term.
#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    /// A variable, represented by its name.
    Var(String),
    /// An abstraction (lambda function) that binds a variable in its body.
    Abs { var: String, body: Box<Term> },
    /// An application of one term to another.
    App(Box<Term>, Box<Term>),
}

use Term::*;

impl Term {
    /// Checks whether the term is complete given a set of bound variable names.
    ///
    /// A term is considered *complete* if every variable that occurs in the term
    /// is bound by some abstraction or is present in the supplied context `ctx`.
    ///
    /// The rules are as follows:
    ///
    /// - **Variable (`Var`)**: A variable is complete if its name exists in `ctx`.
    /// - **Abstraction (`Abs`)**: The abstraction adds its bound variable to the context
    ///   and then checks whether its body is complete under this extended context.
    /// - **Application (`App`)**: In this implementation, an application is considered
    ///   complete if **both** of its subterms is complete under the given context.
    ///
    /// # Examples
    ///
    /// A variable is complete only if it is present in the context:
    /// ```
    /// # use application::term::util::*;
    /// let ctx = std::collections::HashSet::from(["x".to_string()]);
    /// assert!(var("x").is_complete_with(ctx.clone()));
    /// assert!(!var("y").is_complete_with(ctx));
    /// ```
    ///
    /// An abstraction binds its variable, so its body is checked with the bound variable added:
    /// ```
    /// # use application::term::util::*;
    /// assert!(abs("x", "x").is_complete_with(std::collections::HashSet::new()));
    /// ```
    ///
    /// For an application, both subterms must be complete:
    /// ```
    /// # use application::term::util::*;
    /// let ctx = std::collections::HashSet::from(["x".to_string()]);
    /// assert!(app("x", abs("y", "y")).is_complete_with(ctx.clone()));
    /// assert!(!app("x", abs("y", "y")).is_complete_with(std::collections::HashSet::new()));
    /// assert!(!app("x", abs("y", "z")).is_complete_with(ctx));
    /// ```
    pub fn is_complete_with(&self, ctx: HashSet<String>) -> bool {
        match self {
            Var(name) => ctx.contains(name),
            Abs { var, body } => {
                let mut new_ctx = ctx.clone();
                new_ctx.insert(var.clone());
                body.is_complete_with(new_ctx)
            }
            App(t1, t2) => t1.is_complete_with(ctx.clone()) && t2.is_complete_with(ctx),
        }
    }

    /// Checks whether the term is complete, i.e. contains no free variables.
    ///
    /// This function uses [`Self::is_complete_with`] to check that the term is
    /// complete in an empty context, i.e. every variable in the term is bound by
    /// some abstraction.
    ///
    /// # Examples
    ///
    /// A lambda abstraction is complete since it binds its variable:
    /// ```
    /// # use application::term::util::*;
    /// assert!(abs("x", "x").is_complete());
    /// ```
    ///
    /// A free variable is not complete:
    /// ```
    /// # use application::term::util::*;
    /// assert!(!var("x").is_complete());
    /// ```
    ///
    /// An application complete only if both subterms are complete:
    /// ```
    /// # use application::term::util::*;
    /// assert!(!app("x", "y").is_complete());
    /// assert!(app(abs("x", "x"), abs("y", "y")).is_complete());
    /// assert!(!app(abs("x", "x"), abs("y", "x")).is_complete());
    /// ```
    pub fn is_complete(&self) -> bool {
        self.is_complete_with(HashSet::new())
    }

    //My other functions
    pub fn subst(self, x: &str, v: Self) -> Self {
        match self {
            Var(name) => {
                if name == x {
                    v
                } else {
                    Term::Var(name)
                }
            }
            Abs { var, body } => {
                if var == x {
                    Term::Abs { var, body }
                } else {
                    Term::Abs {
                        var,
                        body: Box::new(body.subst(x, v)),
                    }
                }
            }
            App(t1, t2) => Term::App(Box::new(t1.subst(x, v.clone())), Box::new(t2.subst(x, v))),
        }
    }

    pub fn free(&self, x: &str) -> bool {
        match self {
            Var(v) => v == x,
            Abs { var, body } => {
                if var == x {
                    false
                } else {
                    body.free(x)
                }
            }
            App(t1, t2) => t1.free(x) || t2.free(x),
        }
    }
}
