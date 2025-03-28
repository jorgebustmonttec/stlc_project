pub use super::Term::{self, *};
pub use crate::r#type::util::*;

pub fn var(name: impl ToString) -> Term {
    Var(name.to_string())
}

// Implicitly converts strings to variable terms
impl From<&str> for Box<Term> {
    fn from(var: &str) -> Self {
        Box::new(Var(var.to_string()))
    }
}

pub fn abs(var: impl ToString, ty: impl Into<Type>, body: impl Into<Box<Term>>) -> Term {
    Abs {
        var: var.to_string(),
        ty: ty.into(),
        body: body.into(),
    }
}
pub fn app(t1: impl Into<Box<Term>>, t2: impl Into<Box<Term>>) -> Term {
    App(t1.into(), t2.into())
}
pub fn letin(var: impl ToString, val_t: impl Into<Box<Term>>, body: impl Into<Box<Term>>) -> Term {
    Let {
        var: var.to_string(),
        val_t: val_t.into(),
        body: body.into(),
    }
}
pub fn ite(
    cond: impl Into<Box<Term>>,
    if_true: impl Into<Box<Term>>,
    if_false: impl Into<Box<Term>>,
) -> Term {
    Ite {
        cond: cond.into(),
        if_true: if_true.into(),
        if_false: if_false.into(),
    }
}

pub fn id2() -> Term {
    abs("x", Boolean, "x")
}
pub fn id22() -> Term {
    abs("x", arrow(Boolean, Boolean), "x")
}
