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

// Implicitly converts integers to integer terms
impl From<i32> for Box<Term> {
    fn from(int: i32) -> Self {
        Box::new(Int(int))
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

pub fn add(t1: impl Into<Box<Term>>, t2: impl Into<Box<Term>>) -> Term {
    Add(t1.into(), t2.into())
}
pub fn sub(t1: impl Into<Box<Term>>, t2: impl Into<Box<Term>>) -> Term {
    Sub(t1.into(), t2.into())
}
pub fn mul(t1: impl Into<Box<Term>>, t2: impl Into<Box<Term>>) -> Term {
    Mul(t1.into(), t2.into())
}
pub fn eq(t1: impl Into<Box<Term>>, t2: impl Into<Box<Term>>) -> Term {
    Eq(t1.into(), t2.into())
}
pub fn ne(t1: impl Into<Box<Term>>, t2: impl Into<Box<Term>>) -> Term {
    Ne(t1.into(), t2.into())
}
pub fn lt(t1: impl Into<Box<Term>>, t2: impl Into<Box<Term>>) -> Term {
    Lt(t1.into(), t2.into())
}
pub fn le(t1: impl Into<Box<Term>>, t2: impl Into<Box<Term>>) -> Term {
    Le(t1.into(), t2.into())
}
pub fn gt(t1: impl Into<Box<Term>>, t2: impl Into<Box<Term>>) -> Term {
    Gt(t1.into(), t2.into())
}
pub fn ge(t1: impl Into<Box<Term>>, t2: impl Into<Box<Term>>) -> Term {
    Ge(t1.into(), t2.into())
}

pub fn id2() -> Term {
    abs("x", Boolean, "x")
}
pub fn id22() -> Term {
    abs("x", arrow(Boolean, Boolean), "x")
}
