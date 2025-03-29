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

    // ==============================Lambda stuff==============================

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

    // ==============================Let stuff==============================

    /// A let expression assigning a variable `var` to a value `val_t` in `body`.
    /// It is effectively just a subtitution.
    Let {
        var: String,
        val_t: Box<Term>,
        body: Box<Term>,
    },

    // ==============================Boolean stuff==============================

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

    // ==============================Integer stuff==============================

    /// An integer value
    Int(i32),
    /// Addition of two terms
    Add(Box<Term>, Box<Term>),
    /// Subtraction of two terms
    Sub(Box<Term>, Box<Term>),
    /// Multiplication of two terms
    Mul(Box<Term>, Box<Term>),

    // ============================Comparison stuff============================

    /// Equality comparison
    Eq(Box<Term>, Box<Term>),
    /// Non-equality
    Ne(Box<Term>, Box<Term>),
    /// Less than
    Lt(Box<Term>, Box<Term>),
    /// Less than or equal
    Le(Box<Term>, Box<Term>),
    /// Greater than
    Gt(Box<Term>, Box<Term>),
    /// Greater than or equal
    Ge(Box<Term>, Box<Term>),


    // ============================Pair stuff============================

    /// A pair consisting of terms
    Pair(Box<Term>, Box<Term>),
    /// The first term in the pair
    Fst(Box<Term>),
    /// The second term in the pair
    Snd(Box<Term>),

    // ===========================List stuff============================

     /// An empty list of some item type
     Nil(Type),
     /// The recursive constructor for lists, holds the head and the tail in the following order: `Cons(head, tail)`.
     Cons(Box<Term>, Box<Term>),
     /// Case analysis for lists
     ///
     /// ```text
     /// lcase t of
     /// | nil => nil_t
     /// | cons cons_var tail_var => cons_t
     /// ```
     LCase {
         t: Box<Term>,
         nil_t: Box<Term>,
         head_var: String,
         tail_var: String,
         cons_t: Box<Term>,
     },
 

    
}

use Term::*;

impl Term {
    /// Determines whether the term is a value.
    ///
    /// Lambda abstractions [`Term::Abs`], [`Term::True`], [`Term::False`] and [`Term::Int`] are considered values.
    /// Let expressions, if-then-else expressions, comparison operators, applications, and variables are not values.
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
        //todo!()

        match self {
            Abs { .. } | True | False | Int(_) => true,
            Pair(t1, t2) => t1.is_value() && t2.is_value(),

            _ => false,
        }
    }
}
