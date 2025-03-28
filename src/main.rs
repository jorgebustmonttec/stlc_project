/*

STLC + 2
0 / 40 points

There are going to be a few changes to the project structure. To keep things organized, the starter code is split into more files. Notably substitution and evaluation related code are in their own files.




The starter code is organized as follows:

term.rs: contains the definition for enum Term and the is_value method.
term/subst.rs: contains the subst method for core STLC terms.
term/step.rs: contains the step and multistep methods for core STLC terms.
term/parse.rs: contains the parsing code for terms.
term/display.rs: contains the pretty printer for terms.
term/util.rs: contains some utilities for working with terms.
type.rs: contains the definition for enum Type and enum TypeError.
type/check.rs: contains the type checking code for core STLC.
type/parse.rs: contains the parsing code for types.
type/display.rs: contains the pretty printer for types.
type/util.rs: contains some utilities for working with types.
parse.rs: contains a few common parsing utilities.
You are free to organize your code in any way you wish, however we recommend using the same structure as in the starter code.
Exercise task
Your task is to implement the dynamics and statics of booleans on top of the core STLC (with let expressions), whose implementation you will find in the starter code. Either use the starter code as the starting point or make the following changes to your version of the project:

Extend Term with boolean values and if-then-else:
enum Term {
    // ..

<pre><code>/// A true boolean value
True,
/// A false boolean value
False,
/// If-then-else
Ite {
    cond: Box&lt;Term&gt;,
    if_true: Box&lt;Term&gt;,
    if_false: Box&lt;Term&gt;,
},
</code>
}

Copy the type checking code from the starter and the module pub mod r#type; into your lib.rs.
Copy the parsing code from parse.rs and pretty printer code display.rs from the respective directories for terms and types.
Refactor the substitution and evaluation methods into their own files (recommended but not necessary).
The grader only tests is_value, subst, step, multistep and type_check and does not test parsing or utilities.




Here is a sample from the REPL:

> True
True :: 𝟚
> let not = (fun t : Boolean, if t then False else True) in not
𝜆 t : 𝟚. if t then False else True :: 𝟚 → 𝟚
> let not = (fun t : Boolean, if t then False else True) in not True
False :: 𝟚
> let not = (fun t : Boolean, if t then False else True) in not False
True :: 𝟚
> let not = (fun t : Boolean, if t then False else True) in fun x : Boolean -> Boolean -> Boolean, x True
𝜆 x : 𝟚 → 𝟚 → 𝟚. x True :: (𝟚 → 𝟚 → 𝟚) → 𝟚 → 𝟚
# And here are a few type errors
> if True then True else x
undefined variable: x
> True False
wrong app type left: Boolean
> (fun x : Boolean -> Boolean, x) True
wrong app type right: Boolean
> if True then True else (fun x : Boolean, x)
type error
> let not = (fun t : Boolean, if t then False else True) in (fun x : (Boolean -> Boolean) -> Boolean, x not) not
wrong app type right: Arrow(Boolean, Boolean)


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
