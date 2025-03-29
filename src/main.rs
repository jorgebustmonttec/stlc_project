/*Sums
0 / 25 points

Your task is to implement the dynamics and statics of sums on top of STLC + ℤ + 𝟚 + pairs + lists according to the evaluation and typing rules presented in the material.

Extend Term with sums:
    /// Injection to the left with the type of the right
    Inl(Box<Term>, Type),
    /// Injection to the right with the type of the left
    Inr(Box<Term>, Type),
    /// Case analysis for sum types
    ///
    /// ```text
    /// lcase t of
    /// | inl inl_var => inl_t
    /// | inr inr_var => inr_t
    /// ```
    Case {
        t: Box<Term>,
        inl_var: String,
        inl_t: Box<Term>,
        inr_var: String,
        inr_t: Box<Term>,
    },
Extend Type with lists:
    /// Type of sums
    Sum(Box<Type>, Box<Type>),
From the starter code, copy the parsing code (parse.rs) and pretty printer code (display.rs) from the respective directories for terms and types. The parsing code supports creating sums with inl t T, inr t T and the case expression case s of | inl x => t1 | inr y => t2. Note that inl t T, inr t T usually need to be surrounded in parentheses. The sum type consisting of either an integer or a boolean is written as Integer + Boolean.
The grader only tests the is_value, subst, step, multistep and type_check methods and does not test parsing or utilities.




Here is a sample from the REPL:

> case inl (2, 3) Integer of | inl x => fst x | inr y => y
2 :: ℤ
> let toint = (fun x : Boolean + Integer, case x of | inl b => if b then 1 else 0 | inr i => i) in toint
𝜆 x : 𝟚 + ℤ. case x of | inl b ⇒ if b then 1 else 0 | inr i ⇒ i :: 𝟚 + ℤ → ℤ
> let toint = (fun x : Boolean + Integer, case x of | inl b => if b then 1 else 0 | inr i => i) in toint (inl True Integer)
1 :: ℤ
> let toint = (fun x : Boolean + Integer, case x of | inl b => if b then 1 else 0 | inr i => i) in toint (inr 5 Boolean)
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
