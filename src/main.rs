/*

Multi-Step

Implement the multistep function as described in the material (Definition 6.4.3).




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
Your task is to implement the multistep method according to the material.

impl Term {
    pub fn multistep(self) -> Self {
        todo!()
    }
}
The grader only tests the multistep method.




To start the REPL, run cargo run. Here is a sample (with Church encoding of pairs) from how the REPL works:

# This is the church encoding of the pair of functions (𝜆 x. x, 𝜆 y. y)
> (fun x, fun y, fun z, z x y) (fun x, x) (fun y, y)
       ((𝜆 x. 𝜆 y. 𝜆 z. (z x) y) (𝜆 x. x)) (𝜆 y. y)
  -->* 𝜆 z. (z (𝜆 x. x)) (𝜆 y. y)
# This gets the first value from the pair
> (fun p, p (fun x, fun y, x)) ((fun x, fun y, fun z, z x y) (fun x, x) (fun y, y))
       (𝜆 p. p (𝜆 x. 𝜆 y. x)) (((𝜆 x. 𝜆 y. 𝜆 z. (z x) y) (𝜆 x. x)) (𝜆 y. y))
  -->* 𝜆 x. x
# This gets the second value from the pair
> (fun p, p (fun x, fun y, y)) ((fun x, fun y, fun z, z x y) (fun x, x) (fun y, y))
       (𝜆 p. p (𝜆 x. 𝜆 y. y)) (((𝜆 x. 𝜆 y. 𝜆 z. (z x) y) (𝜆 x. x)) (𝜆 y. y))
  -->* 𝜆 y. y
# The Y combinator applied to itself leads to infinite recursion
> (fun x, x x) (fun x, x x)
       (𝜆 x. x x) (𝜆 x. x x)
Feel free to try out different lambda terms from Church encoding WikiPedia page.

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
