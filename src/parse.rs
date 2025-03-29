use nom::{
    character::complete::{multispace0, multispace1},
    sequence::delimited,
    Parser,
};

pub fn ws0<'a, F: 'a, O>(
    inner: F,
) -> impl Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>
where
    F: Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>,
{
    delimited(multispace0, inner, multispace0)
}

pub fn ws1<'a, F: 'a, O>(
    inner: F,
) -> impl Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>
where
    F: Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>,
{
    delimited(multispace1, inner, multispace1)
}
