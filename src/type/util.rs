pub use super::Type::{self, *};

pub fn arrow(ty1: impl Into<Box<Type>>, ty2: impl Into<Box<Type>>) -> Type {
    Arrow(ty1.into(), ty2.into())
}
pub fn prod(ty1: impl Into<Box<Type>>, ty2: impl Into<Box<Type>>) -> Type {
    Prod(ty1.into(), ty2.into())
}
