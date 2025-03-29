use nom::character::complete::multispace1;
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

fn parse_arrow_type(input: &str) -> IResult<&str, Type> {
    let (rest, ty1) = parse_type_primary.parse(input)?;

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

fn parse_list_type(input: &str) -> IResult<&str, Type> {
    (tag("List"), multispace1, parse_type_primary) // Only base or paren allowed
        .map(|(_, _, ty)| List(ty.into()))
        .parse(input)
}

fn parse_list_type_parens(input: &str) -> IResult<&str, Type> {
    delimited(char('['), ws0(parse_type), char(']'))
        .map(|ty| List(ty.into()))
        .parse(input)
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
