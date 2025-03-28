/*

Arithmetic and comparisons
0 / 40 points

The starter code is organized like in the previous exercise (STLC + 2), and contains a parser and utilities for booleans.




Your task is to implement the dynamics and statics of integers and arithmetic on top of STLC + 2 according to the evaluation and typing rules presented in the material. The starter code contains the solution to the previous exercise with holes added for you to fill in. Either use the starter code as the starting point or make the following changes to your version of the project:

Extend Term with integers, arithmetic and comparison operators
    /// An integer value
    Int(i32),
    /// Addition of two terms
    Add(Box<Term>, Box<Term>),
    /// Subtraction of two terms
    Sub(Box<Term>, Box<Term>),
    /// Multiplication of two terms
    Mul(Box<Term>, Box<Term>),

<pre><code>/// Equality comparison
Eq(Box&lt;Term&gt;, Box&lt;Term&gt;),
/// Non-equality
Ne(Box&lt;Term&gt;, Box&lt;Term&gt;),
/// Less than
Lt(Box&lt;Term&gt;, Box&lt;Term&gt;),
/// Less than or equal
Le(Box&lt;Term&gt;, Box&lt;Term&gt;),
/// Greater than
Gt(Box&lt;Term&gt;, Box&lt;Term&gt;),
/// Greater than or equal
Ge(Box&lt;Term&gt;, Box&lt;Term&gt;),
</code>
Extend Type with integers:
pub enum Type {
    Boolean,
    Integer,

    /// Type of abstractions
    Arrow(Box<Type>, Box<Type>),
}
From the starter code, copy the parsing code (parse.rs) and pretty printer code (display.rs) from the respective directories for terms and types. The parsing code implements left-associative infix parsing for the common operators +, -, *, ==, != <, <=, >, >= with natural order of operations.
The grader only tests step, multistep and type_check and does not test parsing or utilities.




Here is a sample from the REPL:

> let a = 5 in a + 2
7 :: ℤ
> let a = 5 in if a < 7 then a + 7 else False
type error
> let a = 5 in if a < 7 then a + 7 else a
12 :: ℤ
> let f = (fun x : Integer, 2 * x) in 1 + f 4 - 8
1 :: ℤ
> let f = (fun x : Integer, f x) in f
undefined variable: f


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
