/*
Fix
0 / 25 points

Your task is to implement the dynamics and statics of fixed points on top of STLC + ℤ + 𝟚 + pairs + lists + sums according to the material in this chapter.

Extend Term with the fixed point combinator:
    /// Fixed point combinator.
    /// Calculates the fixed point of the inner function.
    Fix(Box<Term>),
Type doesn't need new variants.
From the starter code, copy the parsing code (parse.rs) and pretty printer code (display.rs) from the respective directories for terms and types.
The grader only tests the is_value, subst, step, multistep and type_check methods and does not test parsing or utilities.




Here is a sample from the REPL:

> let fibGen = (fun fib : Integer -> Integer, fun n : Integer, if n <= 1 then n else fib (n - 1) + fib (n - 2)) in fibGen
𝜆 fib : ℤ → ℤ. 𝜆 n : ℤ. if n <= 1 then n else fib (n - 1) + fib (n - 2) :: (ℤ → ℤ) → ℤ → ℤ
# Notice that `fix fibGen` can be reduced immediately
> let fibGen = (fun fib : Integer -> Integer, fun n : Integer, if n <= 1 then n else fib (n - 1) + fib (n - 2)) in fix fibGen
𝜆 n : ℤ. if n <= 1 then n else (fix 𝜆 fib : ℤ → ℤ. 𝜆 n : ℤ. if n <= 1 then n else fib (n - 1) + fib (n - 2)) (n - 1) + (fix 𝜆 fib : ℤ → ℤ. 𝜆 n : ℤ. if n <= 1 then n else fib (n - 1) + fib (n - 2)) (n - 2) :: ℤ → ℤ
> let fibGen = (fun fib : Integer -> Integer, fun n : Integer, if n <= 1 then n else fib (n - 1) + fib (n - 2)) in (fix fibGen) 10
55 :: ℤ
> let factGen = (fun fact : Integer -> Integer, fun n : Integer, if n <= 1 then 1 else n * fact (n - 1)) in (fix factGen) 10
3628800 :: ℤ
As a challenge, try defining the following functions even :: ℤ → ℤ, len :: [ℤ] → ℤ, sum :: [ℤ] → ℤ, map :: (ℤ → ℤ) → [ℤ] → [ℤ]. However, no points are awarded for completing this challenge.
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
