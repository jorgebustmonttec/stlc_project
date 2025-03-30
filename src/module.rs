//! # Modules
//!
//! Modules are collections of declarations.
//! A declaration consists of a name, a type and a term.
//! A declaration may refer to any previous declaration or to itself recursively.
//!
//! For example the following code represents a module:
//!
//! ```stlc
//! x : Integer
//! x = 5
//!
//! y : Integer
//! y = x
//! ```
//!
//! The main entrypoint to using a module is [`Module::to_term`].

use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use nom::combinator::all_consuming;
use nom::Parser;

use crate::{r#type::Type, term::Term};
use parse::parse_module;
use Term::*;

pub mod parse;

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration(pub String, pub Type, pub Term);

impl Declaration {
    /// Converts a recursive declaration to a [`Term::Let`] with a [`Term::Fix`] inside. Takes in the inner body term.
    ///
    /// ## Examples
    ///
    /// The declaration
    ///
    /// ```stlc
    /// x : Integer
    /// x = 5
    /// ```
    ///
    /// Is converted to a `let x = fix (fun x : Integer, 5) in <body>` with the argument `body` filled in.
    pub fn to_term_fix(self, body: Term) -> Term {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
/// An import statement.
///
/// ```stlc
/// import a.b
/// ```
///
/// imports the file `a/b.stlc`.
pub struct Import(pub PathBuf);

impl Import {
    /// Reads the imported file and converts it to a term.
    pub fn read_to_term(
        &self,
        basepath: impl AsRef<Path>,
        body: Term,
    ) -> Result<Term, Box<dyn std::error::Error>> {
        let import_path = basepath.as_ref().join(&self.0);
        let import_base = import_path.parent().expect("import to have a parent");
        let code = read_to_string(&import_path)?;
        let result = all_consuming(parse_module)
            .parse(&code)
            .map_err(|err| err.to_string())?
            .1
            .to_term(&import_base, body);
        result
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Module(pub Vec<Import>, pub Vec<Declaration>);

impl Module {
    /// Returns an empty module.
    pub fn new() -> Self {
        Self::default()
    }

    /// Converts a module to a [`Term::Fix`]. Takes in the inner body term.
    ///
    /// # Examples
    ///
    /// A module with one declaration
    ///
    /// ```stlc
    /// x : Integer
    /// x = 5
    /// ```
    ///
    /// is converted to a `let x = fix fun x : Integer, 5 in [body]`.
    ///
    /// A module with two declarations
    ///
    /// ```stlc
    /// x : Integer
    /// x = 5
    ///
    /// y : Integer
    /// y = x
    /// ```
    ///
    /// is converted to
    /// ```stlc
    /// let x = fix fun x : Integer, 5 in
    /// let y = fix fun y : Integer, x in
    /// in [body]
    /// ```
    ///
    /// The `basepath` should be set to the module's parent directory in order for imports to work properly.
    pub fn to_term(
        self,
        basepath: impl AsRef<Path>,
        body: Term,
    ) -> Result<Term, Box<dyn std::error::Error>> {
        let mut term = body;
        // Process declarations from bottom to top
        for decl in self.1.into_iter().rev() {
            todo!()
        }
        // Process imports from bottom to top
        for import in self.0.into_iter().rev() {
            todo!()
        }
        Ok(term)
    }
}
