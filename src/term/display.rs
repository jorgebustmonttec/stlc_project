use super::Term;

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Term::*;
        match self {
            Var(x) => write!(f, "{x}"),
            Abs { var, body } => write!(f, "𝜆 {var}. {body}"),
            App(term1, term2) => match (&**term1, &**term2) {
                (term1 @ Var(_), term2 @ Var(_)) => {
                    // Applying variables doesn't need parens
                    write!(f, "{term1} {term2}")
                }
                (term1 @ Var(_), term2) => {
                    // Applying a variable on the left to anything
                    write!(f, "{term1} ({term2})")
                }
                (term1, term2 @ Var(_)) => {
                    // Applying anything to a variable
                    write!(f, "({term1}) {term2}")
                }
                // Otherwise add parens
                _ => write!(f, "({term1}) ({term2})"),
            },
        }
    }
}
