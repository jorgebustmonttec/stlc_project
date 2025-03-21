use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0, char, multispace0, multispace1},
    combinator::verify,
    multi::fold_many0,
    sequence::delimited,
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq)]
enum Term {
    Var(String),
    App(Box<Term>, Box<Term>),
    Abs { var: String, body: Box<Term> },
}

use Term::*;

// Term utilities

impl From<&str> for Term {
    fn from(var: &str) -> Self {
        Var(var.to_string())
    }
}

fn fun(var: impl AsRef<str>, body: impl Into<Term>) -> Term {
    Abs {
        var: var.as_ref().into(),
        body: body.into().into(),
    }
}
fn app(t1: impl Into<Term>, t2: impl Into<Term>) -> Term {
    App(t1.into().into(), t2.into().into())
}

// Actual parser

pub fn parse_variable_name(input: &str) -> IResult<&str, String> {
    verify(
        (alpha1, alphanumeric0).map(|(s1, s2)| format!("{s1}{s2}")),
        |name: &str| name != "fun",
    )
    .parse(input)
}

fn parse_var(input: &str) -> IResult<&str, Term> {
    parse_variable_name.map(Var).parse(input)
}

fn parse_abs(input: &str) -> IResult<&str, Term> {
    (
        tag("fun"),
        multispace1,
        parse_variable_name,
        multispace0,
        char(','),
        multispace0,
        parse_term,
    )
        .map(|(_0, _1, var, _3, _4, _5, body)| Abs {
            var,
            body: body.into(),
        })
        .parse(input)
}

fn parse_paren(input: &str) -> IResult<&str, Term> {
    delimited(char('('), parse_term, char(')')).parse(input)
}

fn parse_app(input: &str) -> IResult<&str, Term> {
    let (rest, t1) = alt((parse_paren, parse_var, parse_abs)).parse(input)?;

    todo!()
}

fn parse_term(input: &str) -> IResult<&str, Term> {
    alt((parse_app, parse_paren, parse_var, parse_abs)).parse(input)
}

fn main() {
    println!("Running tests for parse_term");

    assert_eq!(parse_term("fun x, x"), Ok(("", fun("x", "x"))));
    assert_eq!(parse_term("fun x, f x"), Ok(("", fun("x", app("f", "x")))));
    assert_eq!(parse_term("a b c"), Ok(("", app(app("a", "b"), "c"))));
    assert_eq!(parse_term("a (b c)"), Ok(("", app("a", app("b", "c")))));

    assert_eq!(
        parse_term("a (b c) d"),
        Ok(("", app(app("a", app("b", "c")), "d")))
    );
    assert_eq!(
        parse_term("(fun x, x) x"),
        Ok(("", app(fun("x", "x"), "x")))
    );
    assert_eq!(
        parse_term("a (fun x, x)"),
        Ok(("", app("a", fun("x", "x"))))
    );

    println!("All tests passed!");
}
