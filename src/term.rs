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
    // 'Free variables' functions

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

    // 'Substitution' functions

    /// Performs substitution of a variable `x` with a given term `v`.
    ///
    /// # Incomplete Terms
    ///
    /// If `v` contains free variables, the function's behavior remains well-defined
    /// but the correctness of the result is not guaranteed, i.e. this function can assume `v`
    /// is a complete term, but must not panic even if it's not.
    ///
    /// # Examples
    ///
    /// ```
    /// # use application::term::util::*;
    /// assert_eq!(var("x").subst("x", var("y")), var("y"));
    /// ```
    ///
    /// ```
    /// # use application::term::util::*;
    /// assert_eq!(abs("x", "x").subst("x", var("y")), abs("x", "x"));
    /// ```
    pub fn subst(self, x: &str, v: Self) -> Self {
        match self {
            Var(y) if y == x => v,
            Abs { var, body } if var != x => Abs {
                var,
                body: Box::new(body.subst(x, v)),
            },
            App(t1, t2) => App(Box::new(t1.subst(x, v.clone())), Box::new(t2.subst(x, v))),
            _ => self,
        }
    }

    // 'Term completeness' functions

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

    // 'Beta reduction step' functions

    /// Performs one step of 𝛽-reduction on the term.
    ///
    /// # Examples
    ///
    /// ```
    /// # use application::term::util::*;
    /// assert_eq!(app(id(), id()).step(), id());
    /// assert_eq!(app(tru(), app(id(), fals())).step(), app(tru(), fals()));
    /// ```
    ///
    /// # Errors
    ///
    /// This function will panic in the following cases:
    /// - If trying to step a variable term.
    /// - If trying to step a term that is already a value.
    ///
    /// The error messages must include "cannot evaluate a variable" and "cannot step a value" respectively.
    ///
    /// ```should_panic
    /// # use application::term::util::*;
    /// id().step();
    /// ```
    pub fn step(self) -> Self {
        match self {
            Var(y) => panic!("cannot evaluate a variable: {y}"),
            Abs { .. } => panic!("cannot step a value"),
            App(t1, t2) => match *t1 {
                // AppAbs: applying an abstraction to a value results in a substitution
                Abs { var, body } if t2.is_value() => body.subst(&var, *t2),
                // App2: if t1 is a value, reduce t2 by one step
                t1 if t1.is_value() => App(Box::new(t1), Box::new(t2.step())),
                // App1: reduce t1 by one step
                t1 => App(Box::new(t1.step()), t2),
            },
        }
    }

    /// Determines whether the term is a value.
    ///
    /// In lambda calculus, only abstractions are considered values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use application::term::util::*;
    /// assert!(abs("x", "x").is_value());
    /// assert!(!var("x").is_value());
    /// assert!(!app("x", "y").is_value());
    /// ```
    pub fn is_value(&self) -> bool {
        match self {
            Abs { .. } => true,
            Var(_) | App(_, _) => false,
            _ => todo!(),
        }
    }

    // 'multi-step' functions

    /// Performs repeated 𝛽-reduction steps until the term reaches a value.
    ///
    /// This method implements a multi-step evaluation strategy where the term is continuously
    /// reduced using the single-step reduction defined in [`Self::step`] until it becomes a value,
    /// as determined by [`Self::is_value`]. In our lambda calculus, only abstractions (i.e., lambda
    /// functions) are considered values.
    ///
    /// # Minor Considerations
    ///
    /// - This method will run indefinitely if the term diverges during evaluation.
    ///   It is the caller's responsibility to ensure that the term reduces to a value.
    /// - **Evaluation Strategy:** This method applies reductions in a loop (a form of big-step
    ///   evaluation) rather than producing an explicit sequence of intermediate terms.
    ///
    /// # Examples
    ///
    /// Evaluating the identity function applied to itself:
    /// ```
    /// # use application::term::util::*;
    /// let term = app(id(), id());
    /// let result = term.multistep();
    /// assert_eq!(result, id());
    /// ```
    ///
    /// Evaluating a nested application:
    /// ```
    /// # use application::term::util::*;
    /// let term = app(tru(), app(id(), fals()));
    /// let result = term.multistep();
    /// assert_eq!(result, abs("y", fals()));
    /// ```
    pub fn multistep(self) -> Self {
        //todo!()

        let mut term = self;
        while !term.is_value() {
            term = term.step();
        }
        term
    }
}
