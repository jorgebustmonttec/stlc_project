pub use super::Term::{self, *};

pub fn var(name: impl ToString) -> Term {
    Var(name.to_string())
}

// Implicitly converts strings to variable terms
impl From<&str> for Box<Term> {
    fn from(var: &str) -> Self {
        Box::new(Var(var.to_string()))
    }
}

pub fn abs(var: impl ToString, body: impl Into<Box<Term>>) -> Term {
    Abs {
        var: var.to_string(),
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
pub fn id() -> Term {
    abs("x", "x")
}
pub fn tru() -> Term {
    abs("x", abs("y", "x"))
}
pub fn fals() -> Term {
    abs("x", abs("y", "y"))
}
