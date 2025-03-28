use super::Term;

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Term::*;
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
        }
    }
}
