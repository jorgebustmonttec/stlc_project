use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0, char, digit1, multispace0, multispace1},
    combinator::{value, verify},
    multi::fold_many0,
    sequence::delimited,
    IResult, Parser,
};

use super::Term::{self, *};
use crate::parse::*;
use crate::r#type::parse::*;

pub fn parse_variable_name(input: &str) -> IResult<&str, String> {
    verify(
        (alpha1, alphanumeric0).map(|(s1, s2)| format!("{s1}{s2}")),
        |name: &str| {
            ![
                "fun", "let", "in", "if", "then", "else", "True", "False", "Integer", "Boolean",
                "fst", "snd",
            ]
            .contains(&name)
        },
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
        char(':'),
        multispace0,
        parse_type,
        multispace0,
        char(','),
        multispace0,
        parse_term,
    )
        .map(|(_0, _1, var, _3, _4, _5, ty, _6, _7, _8, body)| Abs {
            var,
            ty,
            body: body.into(),
        })
        .parse(input)
}

fn parse_paren(input: &str) -> IResult<&str, Term> {
    delimited(char('('), ws0(parse_term), char(')')).parse(input)
}

fn parse_app(input: &str) -> IResult<&str, Term> {
    let (rest, t1) = parse_term_primary.parse(input)?;

    fold_many0(
        (multispace1, parse_term_primary),
        move || t1.clone(),
        |fun, (_, arg)| App(Box::new(fun), Box::new(arg)),
    )
    .parse(rest)
}

fn parse_let(input: &str) -> IResult<&str, Term> {
    (
        tag("let"),
        multispace1,
        parse_variable_name,
        multispace0,
        tag("="),
        multispace0,
        parse_term_primary,
        multispace1,
        tag("in"),
        multispace1,
        parse_term,
    )
        .map(|(_0, _1, var, _3, _4, _5, val_t, _7, _8, _9, body)| Let {
            var,
            val_t: val_t.into(),
            body: body.into(),
        })
        .parse(input)
}

fn parse_int(input: &str) -> IResult<&str, Term> {
    digit1.map_res(str::parse).map(Int).parse(input)
}

fn parse_bool(input: &str) -> IResult<&str, Term> {
    alt((value(True, tag("True")), value(False, tag("False")))).parse(input)
}

fn parse_ite(input: &str) -> IResult<&str, Term> {
    (
        tag("if"),
        ws1(parse_term),
        tag("then"),
        ws1(parse_term),
        tag("else"),
        multispace1,
        parse_term,
    )
        .map(|(_0, cond, _2, if_true, _4, _5, if_false)| Ite {
            cond: cond.into(),
            if_true: if_true.into(),
            if_false: if_false.into(),
        })
        .parse(input)
}

pub fn parse_term_primary(input: &str) -> IResult<&str, Term> {
    alt((parse_paren, parse_var, parse_pair, parse_int, parse_bool)).parse(input)
}

pub fn parse_term(input: &str) -> IResult<&str, Term> {
    alt((
        parse_comparison,
        parse_let,
        parse_ite,
        parse_abs,
        parse_fst,
        parse_snd,
    ))
    .parse(input)
}

fn parse_pair(input: &str) -> IResult<&str, Term> {
    delimited(
        char('('),
        (ws0(parse_term), char(','), ws0(parse_term)),
        char(')'),
    )
    .map(|(t1, _, t2)| Pair(t1.into(), t2.into()))
    .parse(input)
}

fn parse_fst(input: &str) -> IResult<&str, Term> {
    (tag("fst"), multispace1, parse_term_primary)
        .map(|(_, _, t)| Fst(t.into()))
        .parse(input)
}

fn parse_snd(input: &str) -> IResult<&str, Term> {
    (tag("snd"), multispace1, parse_term_primary)
        .map(|(_, _, t)| Snd(t.into()))
        .parse(input)
}

/// Parses a multiplication, which is lower in priority than applications, but higher than +/-
fn parse_mul(input: &str) -> IResult<&str, Term> {
    // Application parser will return a primary term if no applications are found.
    let (rest, t1) = parse_app.parse(input)?;

    fold_many0(
        (ws0(value(Mul, char('*'))), parse_app),
        move || t1.clone(),
        |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
    )
    .parse(rest)
}

/// Parses for +/-, which is lower in priority than multiplication, but higher than comparison operators
fn parse_add_sub(input: &str) -> IResult<&str, Term> {
    let (rest, t1) = parse_mul.parse(input)?;

    fold_many0(
        (
            ws0(alt((
                value(Add as fn(Box<Term>, Box<Term>) -> Term, char('+')),
                value(Sub as fn(Box<Term>, Box<Term>) -> Term, char('-')),
            ))),
            parse_mul,
        ),
        move || t1.clone(),
        |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
    )
    .parse(rest)
}

/// Parses for comparison operators, lowest in priority
fn parse_comparison(input: &str) -> IResult<&str, Term> {
    let (rest, t1) = parse_add_sub.parse(input)?;

    fold_many0(
        (
            ws0(alt((
                value(Eq as fn(Box<Term>, Box<Term>) -> Term, tag("==")),
                value(Ne as fn(Box<Term>, Box<Term>) -> Term, tag("!=")),
                value(Le as fn(Box<Term>, Box<Term>) -> Term, tag("<=")),
                value(Ge as fn(Box<Term>, Box<Term>) -> Term, tag(">=")),
                value(Lt as fn(Box<Term>, Box<Term>) -> Term, tag("<")),
                value(Gt as fn(Box<Term>, Box<Term>) -> Term, tag(">")),
            ))),
            parse_add_sub,
        ),
        move || t1.clone(),
        |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
    )
    .parse(rest)
}
