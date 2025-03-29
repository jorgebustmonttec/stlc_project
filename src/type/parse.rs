use nom::character::complete::multispace1;
use nom::multi::fold_many0;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, combinator::value,
    sequence::delimited, IResult, Parser,
};

use super::super::parse::*;
use super::Type::{self, *};

fn parse_base_type(input: &str) -> IResult<&str, Type> {
    alt((
        value(Boolean, tag("Boolean")),
        value(Integer, tag("Integer")),
    ))
    .parse(input)
}

fn parse_paren_type(input: &str) -> IResult<&str, Type> {
    delimited(char('('), ws0(parse_type), char(')')).parse(input)
}

fn parse_list_type(input: &str) -> IResult<&str, Type> {
    (tag("List"), multispace1, parse_type_primary) // Only base or paren allowed
        .map(|(_, _, ty)| List(Box::new(ty)))
        .parse(input)
}

fn parse_list_type_parens(input: &str) -> IResult<&str, Type> {
    delimited(char('['), ws0(parse_type), char(']'))
        .map(|ty| List(ty.into()))
        .parse(input)
}

/// Parses an arrow type.
///
/// `A -> B + C -> D + E` is parsed as `A -> ((B + C) -> (D + E))`
fn parse_arrow_type(input: &str) -> IResult<&str, Type> {
    let (rest, ty1) = parse_sum_type.parse(input)?;

    if let Ok((rest, (_, ty2))) = (ws0(tag("->")), parse_arrow_type).parse(rest) {
        Ok((rest, Arrow(ty1.into(), ty2.into())))
    } else {
        Ok((rest, ty1))
    }
}

fn parse_prod_type(input: &str) -> IResult<&str, Type> {
    delimited(
        char('('),
        (ws0(parse_type), char(','), ws0(parse_type)),
        char(')'),
    )
    .map(|(ty1, _, ty2)| Prod(ty1.into(), ty2.into()))
    .parse(input)
}

/// Parses a sum type associating to the left.
///
/// `A + B + C` is parsed as `(A + B) + C`
fn parse_sum_type(input: &str) -> IResult<&str, Type> {
    let (rest, ty1) = parse_type_primary.parse(input)?;

    fold_many0(
        (ws0(char('+')), parse_type_primary),
        move || ty1.clone(),
        |lhs, (_, rhs)| Sum(Box::new(lhs), Box::new(rhs)),
    )
    .parse(rest)
}

pub fn parse_type_primary(input: &str) -> IResult<&str, Type> {
    alt((
        parse_base_type,
        parse_prod_type,
        parse_paren_type,
        parse_list_type_parens,
    ))
    .parse(input)
}

pub fn parse_type(input: &str) -> IResult<&str, Type> {
    alt((parse_arrow_type, parse_list_type)).parse(input)
}
