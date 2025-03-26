// Substitution

/*

In this exercise, you will implement substitution. You have two options for getting started:

Use your solution to the previous exercise (Free) as the starting point, or
use the provided starter code.
The starter code comes with the following files:

term.rs: contains the definition for Term and its methods.
lib.rs: makes the term.rs module part of the library crate.
main.rs: a REPL to testing and experimenting. Run cargo add rustyline@15 to add the required dependency for the REPL to work.
term/display.rs: pretty printer for Term.
Your task is to implement the subst method in term.rs according to the material. Its type signature is as follows:

impl Term {
    pub fn subst(self, x: &str, v: Self) -> Self {
        todo!()
    }
}
See Definition 6.3.5 for the exact specification for substitution. The grader only tests the subst method.




To start the REPL, run cargo run. Here is a sample from how the REPL works:

Enter :q or Ctrl+C to quit.
t? x
x? x
v? fun x, x
[x ↦ 𝜆 x. x] x = 𝜆 x. x
t? fun x, y
x? y
v? z
[y ↦ z] 𝜆 x. y = 𝜆 x. z
t? fun x, y
x? x
v? z
[x ↦ z] 𝜆 x. y = 𝜆 x. y

*/

pub mod term;

use crate::term::parse::{parse_term, parse_variable_name};
use crate::term::Term;
use nom::combinator::all_consuming;
use nom::Parser;

use rustyline::{error::ReadlineError, DefaultEditor};

enum State {
    Start,
    HasTerm(Term),
    HasTermVar(Term, String),
}

impl State {
    fn prompt(&self) -> &'static str {
        match self {
            Start => "t? ",
            HasTerm(_) => "x? ",
            HasTermVar(_, _) => "v? ",
        }
    }

    fn process(&self, line: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match self {
            Start => {
                let (_, t) = all_consuming(parse_term)
                    .parse(line)
                    .map_err(|e| e.to_string())?;
                Ok(HasTerm(t))
            }
            HasTerm(t) => {
                let (_, x) = all_consuming(parse_variable_name)
                    .parse(line)
                    .map_err(|e| e.to_string())?;
                Ok(HasTermVar(t.clone(), x))
            }
            HasTermVar(t, x) => {
                let (_, v) = all_consuming(parse_term)
                    .parse(line)
                    .map_err(|e| e.to_string())?;
                print!("[{x} ↦ {v}] {t} = ");
                println!("{}", t.clone().subst(&x, v));
                Ok(Start)
            }
        }
    }
}

use State::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;
    let mut state = Start;
    println!("Enter :q or Ctrl+C to quit.");

    loop {
        let readline = rl.readline(state.prompt());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                if line.trim() == ":q" {
                    break;
                }

                match state.process(&line) {
                    Ok(next_state) => state = next_state,
                    Err(e) => eprintln!("{e}"),
                };
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
