pub mod check;
mod display;
pub mod parse;
pub mod util;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Boolean,

    /// Type of abstractions
    Arrow(Box<Type>, Box<Type>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    UndefinedVariable(String),
    WrongAppTypeRight(Type),
    WrongAppTypeLeft(Type),
    Fail,
}

use TypeError::*;

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UndefinedVariable(x) => write!(f, "undefined variable: {x}"),
            WrongAppTypeRight(ty) => write!(f, "wrong app type right: {ty:?}"),
            WrongAppTypeLeft(ty) => write!(f, "wrong app type left: {ty:?}"),
            Fail => write!(f, "type error"),
        }
    }
}

impl std::error::Error for TypeError {}

pub type Context = std::collections::HashMap<String, Type>;
