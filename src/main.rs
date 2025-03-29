/*
List
0 / 25 points

Your task is to implement the dynamics and statics of lists on top of STLC + ℤ + 𝟚 + pairs according to the evaluation and typing rules presented in the material.

Extend Term with list constructors and list case expressions:
    /// An empty list of some item type
    Nil(Type),
    /// The recursive constructor for lists, holds the head and the tail in the following order: `Cons(head, tail)`.
    Cons(Box<Term>, Box<Term>),
    /// Case analysis for lists
    ///
    /// ```text
    /// lcase t of
    /// | nil => nil_t
    /// | cons cons_var tail_var => cons_t
    /// ```
    LCase {
        t: Box<Term>,
        nil_t: Box<Term>,
        head_var: String,
        tail_var: String,
        cons_t: Box<Term>,
    },
Extend Type with lists:
    /// Type of lists
    List(Box<Ty>),
From the starter code, copy the parsing code (parse.rs) and pretty printer code (display.rs) from the respective directories for terms and types. The parsing code supports creating lists with nil T, cons h t and the lcase expression lcase l of | nil => t1 | cons h t => t2. Note that nil T, cons h t usually need to be surrounded in parentheses. The type of a list of integers can be written with List Integer or [Integer].
The grader only tests the is_value, subst, step, multistep and type_check methods and does not test parsing or utilities.




Here is a sample from the REPL:

> let hdor0 = (fun l : [Integer], lcase l of | nil => 0 | cons h t => h) in hdor0
𝜆 l : [ℤ]. lcase l of | nil ⇒ 0 | cons h t ⇒ h :: [ℤ] → ℤ
> let hdor0 = (fun l : [Integer], lcase l of | nil => 0 | cons h t => h) in hdor0 (nil Integer)
0 :: ℤ
> let hdor0 = (fun l : [Integer], lcase l of | nil => 0 | cons h t => h) in hdor0 (cons 5 (nil Integer))
5 :: ℤ

*/
use nom::Parser;
use nom::combinator::all_consuming;
use stlc_project::term::parse::parse_term;

use rustyline::{DefaultEditor, error::ReadlineError};

fn process(line: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (_, t) = all_consuming(parse_term)
        .parse(line)
        .map_err(|e| e.to_string())?;
    let ty = t.type_check()?;
    println!("{} :: {ty}", t.multistep());
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;
    println!("Enter :q or Ctrl+C to quit.");

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                if line.trim() == ":q" {
                    break;
                }

                if let Err(e) = process(&line) {
                    eprintln!("{e}");
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
