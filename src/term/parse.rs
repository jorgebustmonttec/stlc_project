use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0, char, multispace0, multispace1},
    combinator::verify,
    multi::fold_many0,
    sequence::delimited,
    IResult, Parser,
};

use super::Term::{self, *};

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
    let (rest, t1) = alt((parse_paren, parse_var)).parse(input)?;

    fold_many0(
        (multispace1, alt((parse_paren, parse_var))),
        move || t1.clone(),
        |fun, (_, arg)| App(Box::new(fun), Box::new(arg)),
    )
    .parse(rest)
}

pub fn parse_term(input: &str) -> IResult<&str, Term> {
    alt((parse_app, parse_paren, parse_var, parse_abs)).parse(input)
}
