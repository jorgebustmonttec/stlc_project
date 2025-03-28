use super::r#type::Type;

mod display;
pub mod parse;
pub mod step;
pub mod subst;
pub mod util;

/// Represents a lambda calculus term.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    /// A variable term, represented by a string.
    Var(String),
    /// An abstraction (lambda function) with a variable and a body term.
    Abs {
        var: String,
        ty: Type,
        body: Box<Term>,
    },
    /// An application of one term to another.
    App(Box<Term>, Box<Term>),

    /// A let expression assigning a variable `var` to a value `val_t` in `body`.
    /// It is effectively just a subtitution.
    Let {
        var: String,
        val_t: Box<Term>,
        body: Box<Term>,
    },

    /// A true boolean value
    True,
    /// A false boolean value
    False,
    /// If-then-else
    Ite {
        cond: Box<Term>,
        if_true: Box<Term>,
        if_false: Box<Term>,
    },
}

use Term::*;

impl Term {
    /// Determines whether the term is a value.
    ///
    /// Lambda abstractions [`Term::Abs`], [`Term::True`] and [`Term::False`] are considered values.
    /// Let expressions, if-then-else expressions, applications, and variables are not values.
    ///
    /// # Examples
    ///
    /// **Abstraction is a value:**
    /// ```rust
    /// # use application::term::util::*;
    /// assert!(abs("x", Boolean, var("x")).is_value());
    /// ```
    ///
    /// **True and false are values:**
    /// ```rust
    /// # use application::term::util::*;
    /// assert!(True.is_value());
    /// assert!(False.is_value());
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
    /// assert!(!app(id22(), id2()).is_value());
    /// ```
    ///
    /// **Let expression is not a value:**
    /// ```rust
    /// # use application::term::util::*;
    /// let let_expr = letin("x", id2(), var("x"));
    /// assert!(!let_expr.is_value());
    /// ```
    pub fn is_value(&self) -> bool {
        todo!()
    }
}
