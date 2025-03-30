use super::Term::{self, *};

/// Formats a [`Term::Cons`] recursively as `, 1, 2, 3` and [`Term::Nil`] as `]`.
fn fmt_list(f: &mut std::fmt::Formatter<'_>, t: &Term) -> std::fmt::Result {
    match t {
        Nil(_) => write!(f, "]"),
        Cons(head, tail) => {
            write!(f, ", {head}")?;
            fmt_list(f, tail)
        }
        _ => panic!("expected input to be Nil or Cons"),
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Var(x) => write!(f, "{x}"),
            Abs { var, ty, body } => write!(f, "𝜆 {var} : {ty}. {body}"),
            App(term1, term2) => match (&**term1, &**term2) {
                (term1 @ (Var(_) | True | False), term2 @ (Var(_) | True | False)) => {
                    write!(f, "{term1} {term2}")
                }
                (term1 @ Var(_), term2) => {
                    write!(f, "{term1} ({term2})")
                }
                (term1, term2 @ (Var(_) | True | False)) => {
                    write!(f, "({term1}) {term2}")
                }
                // Otherwise add parens
                _ => write!(f, "({term1}) ({term2})"),
            },
            Let { var, val_t, body } => write!(f, "let {var} = {val_t} in {body}"),
            True => write!(f, "True"),
            False => write!(f, "False"),
            Ite {
                cond,
                if_true,
                if_false,
            } => write!(f, "if {cond} then {if_true} else {if_false}"),
            Int(n) => write!(f, "{n}"),
            Add(term1, term2) => write!(f, "{term1} + {term2}"),
            Sub(term1, term2) => write!(f, "{term1} - {term2}"),
            Mul(term1, term2) => write!(f, "{term1} * {term2}"),
            Eq(term1, term2) => write!(f, "{term1} == {term2}"),
            Ne(term1, term2) => write!(f, "{term1} != {term2}"),
            Lt(term1, term2) => write!(f, "{term1} < {term2}"),
            Le(term1, term2) => write!(f, "{term1} <= {term2}"),
            Gt(term1, term2) => write!(f, "{term1} > {term2}"),
            Ge(term1, term2) => write!(f, "{term1} >= {term2}"),
            Pair(term1, term2) => write!(f, "({term1}, {term2})"),
            Fst(term) => write!(f, "fst {term}"),
            Snd(term) => write!(f, "snd {term}"),
            Nil(_) => write!(f, "[]"),
            Cons(x, xs) => {
                write!(f, "[{x}")?;
                fmt_list(f, xs)
            }
            LCase {
                t,
                nil_t,
                head_var,
                tail_var,
                cons_t,
            } => write!(
                f,
                "lcase {t} of | nil ⇒ {nil_t} | cons {head_var} {tail_var} ⇒ {cons_t}"
            ),
            Inl(term, _) => write!(f, "inl {term}"),
            Inr(term, _) => write!(f, "inr {term}"),
            Case {
                t,
                inl_var,
                inl_t,
                inr_var,
                inr_t,
            } => write!(
                f,
                "case {t} of | inl {inl_var} ⇒ {inl_t} | inr {inr_var} ⇒ {inr_t}"
            ),
            Fix(term) => write!(f, "fix {term}"),
        }
    }
}
