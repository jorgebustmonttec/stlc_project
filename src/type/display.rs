use super::Type::{self, *};

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Boolean => write!(f, "𝟚"),
            Integer => write!(f, "ℤ"),
            Arrow(ty1, ty2) => match (&**ty1, &**ty2) {
                (ty1 @ Arrow(..), ty2) => {
                    // If the left hand side is an arrow, it needs parentheses
                    write!(f, "({ty1}) → {ty2}")
                }
                // Otherwise no parens
                _ => write!(f, "{ty1} → {ty2}"),
            },
        }
    }
}
