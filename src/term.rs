mod display;
pub mod parse;
pub mod util;

/// Represents a lambda calculus term.
#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    /// A variable term, represented by a string.
    Var(String),
    /// An abstraction (lambda function) with a variable and a body term.
    Abs { var: String, body: Box<Term> },
    /// An application of one term to another.
    App(Box<Term>, Box<Term>),
}

impl Term {
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
    /// ```a
    pub fn subst(self, x: &str, v: Self) -> Self {
        match self {
            Term::Var(name) => {
                if name == x {
                    v
                } else {
                    Term::Var(name)
                }
            }
            Term::Abs { var, body } => {
                if var == x {
                    Term::Abs { var, body }
                } else {
                    Term::Abs {
                        var,
                        body: Box::new(body.subst(x, v)),
                    }
                }
            }
            Term::App(t1, t2) => {
                Term::App(Box::new(t1.subst(x, v.clone())), Box::new(t2.subst(x, v)))
            }
        }
    }

    pub fn free(&self, x: &str) -> bool {
        match self {
            Term::Var(v) => v == x,
            Term::Abs { var, body } => {
                if var == x {
                    false
                } else {
                    body.free(x)
                }
            }
            Term::App(t1, t2) => t1.free(x) || t2.free(x),
        }
    }
}
