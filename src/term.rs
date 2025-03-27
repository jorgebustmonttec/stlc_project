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
    /// A let expression assigning a variable `var` to a value `val_t` in `body`.
    /// It is effectively just a subtitution.
    Let {
        var: String,
        val_t: Box<Term>,
        body: Box<Term>,
    },
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
            _ => todo!(),
        }
    }

    // 'Substitution' functions

    /// Performs substitution of a variable `x` with a given term `v`.
    ///
    /// # Incomplete Terms
    ///
    /// If `v` contains free variables, the function's behavior remains well-defined
    /// but the correctness of the result is not guaranteed (i.e. this function assumes that `v`
    /// is complete), though it must not panic even if it is not.
    ///
    /// # Examples
    ///
    /// **Variable substitution:**
    /// ```rust
    /// # use application::term::util::*;
    /// assert_eq!(var("x").subst("x", var("y")), var("y"));
    /// ```
    ///
    /// **Inside an abstraction (bound variable remains unchanged):**
    /// ```rust
    /// # use application::term::util::*;
    /// assert_eq!(abs("x", var("x")).subst("x", var("y")), abs("x", var("x")));
    /// ```
    ///
    /// **Using a let expression:**
    /// In the let expression below, the bound variable is `"y"`. Substituting `"x"` will
    /// affect both the value part and the body.
    /// ```rust
    /// # use application::term::util::*;
    /// let let_expr = letin("y", var("x"), app(var("x"), var("y")));
    /// let expected  = letin("y", var("z"), app(var("z"), var("y")));
    /// assert_eq!(let_expr.subst("x", var("z")), expected);
    /// ```
    ///
    /// Substituting inside the body of a let should only substitute if the var is different.
    /// - `[x ↦ id](let x = id in x) = let x = id in x`, i.e. should be invariant, as `x` is bound by the let.
    /// - `[x ↦ id](let x = x in x) = let x = id in x`, i.e. `val_t` is substituted, as it's not quantified by the let.
    ///
    /// ```rust
    /// # use application::term::util::*;
    /// assert_eq!(
    ///     letin("x", id(), var("x")).subst("x", id()),
    ///     letin("x", id(), var("x"))
    /// );
    /// assert_eq!(
    ///     letin("x", var("x"), var("x")).subst("x", id()),
    ///     letin("x", id(), var("x"))
    /// );
    /// ```
    pub fn subst(self, x: &str, v: Self) -> Self {
        match self {
            Var(y) if y == x => v,
            Abs { var, body } if var != x => Abs {
                var,
                body: Box::new(body.subst(x, v)),
            },
            App(t1, t2) => App(Box::new(t1.subst(x, v.clone())), Box::new(t2.subst(x, v))),
            Let { var, val_t, body } if var != x => Let {
                var,
                val_t: Box::new(val_t.subst(x, v.clone())),
                body: Box::new(body.subst(x, v)),
            },
            Let { var, val_t, body } => Let {
                var,
                val_t: Box::new(val_t.subst(x, v)),
                body, // Do not substitute in the body if the variable matches
            },
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
            _ => todo!(),
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
    /// This function applies one reduction step according to the lambda calculus rules.
    ///
    /// # Examples
    ///
    /// **Application reduction:**
    /// ```rust
    /// # use application::term::util::*;
    /// // (λx. x) applied to (λy. y) reduces to (λy. y)
    /// assert_eq!(app(id(), id()).step(), id());
    /// ```
    ///
    /// **Nested reduction:**
    /// ```rust
    /// # use application::term::util::*;
    /// // In the term app(tru(), app(id(), fals())), the inner application reduces first.
    /// assert_eq!(app(tru(), app(id(), fals())).step(), app(tru(), fals()));
    /// ```
    ///
    /// **Using a let expression:**
    /// When the bound value in a let expression is already a value,
    /// a single step reduces it by substituting the bound variable in the body.
    /// ```rust
    /// # use application::term::util::*;
    /// let let_expr = letin("x", tru(), var("x"));
    /// // This reduces by replacing "x" in the body with tru(), resulting in tru().
    /// assert_eq!(let_expr.step(), tru());
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
                Abs { var, body } if t2.is_value() => body.subst(&var, *t2),
                t1 if t1.is_value() => App(Box::new(t1), Box::new(t2.step())),
                t1 => App(Box::new(t1.step()), t2),
            },
            Let { var, val_t, body } if !val_t.is_value() => Let {
                var,
                val_t: Box::new(val_t.step()),
                body,
            },
            Let { var, val_t, body } => body.subst(&var, *val_t),
        }
    }

    /// Determines whether the term is a value.
    ///
    /// In our lambda calculus, only lambda abstractions (i.e. `Abs`) are considered values.
    /// Let expressions, applications, and variables are not values.
    ///
    /// # Examples
    ///
    /// **Abstraction is a value:**
    /// ```rust
    /// # use application::term::util::*;
    /// assert!(abs("x", var("x")).is_value());
    /// ```
    ///
    /// **Variable is not a value:**
    /// ```rust
    /// # use application::term::util::*;
    /// assert!(!var("x").is_value());
    /// ```
    ///
    /// **Application is not a value:**
    /// ```rust
    /// # use application::term::util::*;
    /// assert!(!app(id(), id()).is_value());
    /// ```
    ///
    /// **Let expression is not a value:**
    /// ```rust
    /// # use application::term::util::*;
    /// let let_expr = letin("x", tru(), var("x"));
    /// assert!(!let_expr.is_value());
    /// ```
    pub fn is_value(&self) -> bool {
        match self {
            Abs { .. } => true,
            Var(_) | App(_, _) | Let { .. } => false,
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
    pub fn multistep(mut self) -> Self {
        while !self.is_value() {
            self = self.step();
        }
        self
    }
}
