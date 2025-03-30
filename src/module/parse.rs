use std::path::PathBuf;

use nom::{
    bytes::complete::tag,
    character::complete::*,
    combinator::all_consuming,
    error::ErrorKind,
    multi::{many1, separated_list1},
    IResult, Parser,
};

use crate::{
    parse::ws0,
    r#type::parse::parse_type,
    term::parse::{parse_term, parse_variable_name},
};

use super::{Declaration, Import, Module};

/// ```stlc
/// a : Integer
/// a = 5
/// ```
///
/// Is converted to a `Declaration("a", Integer, 5)`
pub fn parse_declaration(input: &str) -> IResult<&str, Declaration> {
    (
        parse_variable_name,
        ws0(char(':')),
        (parse_type),
        space0,
        (newline),
        (parse_variable_name),
        ws0(char('=')),
        (parse_term),
    )
        .map_res(|(var1, _, ty, _, _, var2, _, term)| {
            if var1 == var2 {
                Ok(Declaration(var1, ty, term))
            } else {
                Err(ErrorKind::Fail)
            }
        })
        .parse(input)
}

/// ```stlc
/// import a.b
/// ```
///
/// Is converted to a `Import('a/b.stlc')`
pub fn parse_import(input: &str) -> IResult<&str, Import> {
    (
        tag("import"),
        space1,
        separated_list1(char('.'), parse_variable_name),
    )
        .map(|(_, _, mut components)| {
            if let Some(filename) = components.pop() {
                let mut path = PathBuf::new();
                for component in components {
                    path.push(component);
                }
                path.push(format!("{filename}.stlc"));
                Import(path)
            } else {
                unreachable!("separated_list1 must create non empty vec")
            }
        })
        .parse(input)
}

/// ```stlc
/// a : Integer
/// a = 5
///
/// sum : List Integer -> Integer
/// sum = fun xs : List Integer,
///     lcase xs of
///     | nil => 0
///     | cons x xs => x + sum xs
/// ```
///
/// Is converted to a module with two declarations
/// - `Declaration("a", Integer, 5)`
/// - `Declaration("sum", List Integer -> Integer, [...])`
///
/// A module does not have to have imports or declarations.
pub fn parse_module(input: &str) -> IResult<&str, Module> {
    // Split the code at each "empty line"
    let blocks = input.trim().split("\n\n");
    let mut imports = vec![];
    let mut decls = vec![];

    for block in blocks {
        // For each block, try to parse it as an import or a declaration
        let (_, res) = all_consuming(
            many1((parse_import, multispace0).map(|(import, _)| import))
                .map(Ok)
                .or(parse_declaration.map(Err)),
        )
        .parse(block)?;

        // Push the resulting import or declaration to the respective list
        match res {
            Ok(mut import) => imports.append(&mut import),
            Err(decl) => decls.push(decl),
        }
    }
    Ok(("", Module(imports, decls)))
}
