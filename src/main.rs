/*

Pairs
0 / 25 points

Your task is to implement the dynamics and statics of pairs on top of STLC + ℤ + 𝟚 according to the evaluation and typing rules presented in the material. The starter code no longer contains the solution to the previous exercise, but only parsing and other utilities. From this point onwards you are expected to extend your version of the language with the new features.

Extend Term with the pairs and projections:
    /// A pair consisting of terms
    Pair(Box<Term>, Box<Term>),
    /// The first term in the pair
    Fst(Box<Term>),
    /// The second term in the pair
    Snd(Box<Term>),
Extend Type with products:
    /// Product type (Pair)
    Prod(Box<Type>, Box<Type>),
From the starter code, copy the parsing code (parse.rs) and pretty printer code (display.rs) from the respective directories for terms and types. The parsing code supports creating pairs with parenthesis, e.g. (1, 2) and the projection operators fst, snd. Note that applying fst, snd usually needs to be surrounded in parentheses. You may change this behavior in the parser if you wish.
The grader only tests the is_value, subst, step, multistep and type_check methods and does not test parsing or utilities.




Here is a sample from the REPL:

> let p = (fun x : Integer, (x, False), 2) in (fst p) 10
(10, False) :: ℤ × 𝟚
> let add = (fun p : (Integer, Integer), (fst p) + (snd p)) in add
𝜆 p : ℤ × ℤ. fst p + snd p :: ℤ × ℤ → ℤ
> let add = (fun p : (Integer, Integer), (fst p) + (snd p)) in add (2, 5)
7 :: ℤ


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
