/*

Term Completeness

In this exercise, you will implement a function for checking if a term is complete. This exercise is optional in the sense that the next exercise (Beta Reduction Step) in chapter 4 is already available for you.

The starter code comes with the following files:

term.rs: contains the definition for Term and its methods.
lib.rs: makes the term.rs module part of the library crate.
main.rs: a REPL for testing and experimenting.
term/display.rs: pretty printer for Term.
Implement the is_complete_with and is_complete methods for checking whether a term is complete according to the definition in the material.

pub fn is_complete_with(&self, ctx: HashSet<String>) -> bool {
    todo!()
}

pub fn is_complete(&self) -> bool {
    todo!()
}
The grader only tests the is_complete and is_complete_with methods.




To start the REPL, run cargo run. Here is a sample from how the REPL works:

> x
x is not complete
> fun x, x
𝜆 x. x is complete
> fun x, y
𝜆 x. y is not complete
> fun y, fun x, y
𝜆 y. 𝜆 x. y is complete

*/

pub mod term;

use crate::term::parse::parse_term;
use nom::Parser;
use nom::combinator::all_consuming;

use rustyline::{DefaultEditor, error::ReadlineError};

fn process(line: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (_, t) = all_consuming(parse_term)
        .parse(line)
        .map_err(|e| e.to_string())?;
    println!(
        "{t} is {}complete",
        if t.is_complete() { "" } else { "not " }
    );
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
