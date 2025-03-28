use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag, character::complete::char,
    combinator::value, sequence::delimited,
};

use super::Type::{self, *};
use crate::parse::*;

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

pub fn parse_type_primary(input: &str) -> IResult<&str, Type> {
    alt((parse_base_type, parse_paren_type)).parse(input)
}

pub fn parse_type(input: &str) -> IResult<&str, Type> {
    parse_arrow_type.parse(input)
}
