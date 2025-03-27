/*

Beta Reduction Step
0 / 35 points

In this exercise, you will implement beta reduction. This is a significant step in the project. Make sure you understand the material well enough first before implementing the step function.




As before, you have two options for getting started:

Use your solution to the previous exercise (Substitution or Complete) as the starting point, or
use the provided starter code, which is the model solution from the previous exercise with.
The starter code comes with the following files:

term.rs: contains the definition for Term and its methods.
lib.rs: makes the term.rs module part of the library crate.
main.rs: the REPL for testing and experimenting.
term/display.rs: pretty printer for Term.
term/util.rs: utilities for creating new terms.
term/parse.rs: parser for terms.
Your task is to implement the step and is_value methods in term.rs according to the material. Their type signatures are:

impl Term {
    pub fn step(self) -> Self {
        todo!()
    }

    pub fn is_value(&self) -> bool {
        todo!()
    }
}
See Definitions 6.4.1 and 6.4.2 for the exact specification. The grader only tests the step and is_value methods.




To start the REPL, run cargo run. Here is a sample from how the REPL works:

Enter :q or Ctrl+C to quit.
Entering an empty line steps the previous result.
> (fun x, fun y, x) (fun x, fun y, y) (fun x, x)
      ((𝜆 x. 𝜆 y. x) (𝜆 x. 𝜆 y. y)) (𝜆 x. x)
  --> (𝜆 y. 𝜆 x. 𝜆 y. y) (𝜆 x. x)
>
  --> 𝜆 x. 𝜆 y. y
> (fun x, fun y, x) (fun x, fun y, y) (fun x, fun y, x)
      ((𝜆 x. 𝜆 y. x) (𝜆 x. 𝜆 y. y)) (𝜆 x. 𝜆 y. x)
  --> (𝜆 y. 𝜆 x. 𝜆 y. y) (𝜆 x. 𝜆 y. x)
>
  --> 𝜆 x. 𝜆 y. y

*/

use nom::Parser;
use nom::combinator::all_consuming;
use stlc_project::term::{Term, parse::parse_term};

use rustyline::{DefaultEditor, error::ReadlineError};

fn process(line: &str) -> Result<Term, Box<dyn std::error::Error>> {
    let (_, t) = all_consuming(parse_term)
        .parse(line)
        .map_err(|e| e.to_string())?;
    print!("      {t}\n  --> ");
    let u = t.step();
    println!("{u}");
    Ok(u)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;
    let mut t = None::<Term>;
    println!("Enter :q or Ctrl+C to quit.");
    println!("Entering an empty line steps the previous result.");

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                if line.len() == 0 {
                    if let Some(ti) = t {
                        let u = ti.step();
                        println!("  --> {u}");
                        t = Some(u);
                    }
                } else {
                    rl.add_history_entry(line.as_str())?;
                    if line.trim() == ":q" {
                        break;
                    }

                    match process(&line) {
                        Ok(u) => t = Some(u),
                        Err(e) => eprintln!("{e}"),
                    }
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
