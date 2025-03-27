/*

Let expressions

Implement Let expressions to the language as described in the material.


The following change is needed to Term:
enum Term {
    // ..

<pre><code>/// A let expression assigning a variable `var` to a value `val_t` in `body`.
/// It is effectively just a subtitution.
Let {
    var: String,
    val_t: Box&lt;Term&gt;,
    body: Box&lt;Term&gt;,
},
</code>
}

The files in the term directory also have changed, so make sure to copy them to your version as well.
The grader only tests the is_value, subst, step and multistep methods and does not test parsing or utilities.




Here is a sample (with Church encoding of pairs) from the REPL:

> let x = (fun x, x) in x x
       let x = 𝜆 x. x in x x
  -->* 𝜆 x. x
# Shadowing works as expected: the "closer" x is used
> let x = (fun z, z) in let x = (fun w, w) in x
     let x = 𝜆 z. z in let x = 𝜆 w. w in x
-->* 𝜆 w. w
# Defines church encoded pair, fst and snd, and evaluates fst (pair (fun x, x) (fun y, y))
> let pair = (fun x, fun y, fun z, z x y) in let fst = (fun p, p (fun x, fun y, x)) in let snd = (fun p, p (fun x, fun y, y)) in fst (pair (fun x, x) (fun y, y))
       let pair = 𝜆 x. 𝜆 y. 𝜆 z. (z x) y in let fst = 𝜆 p. p (𝜆 x. 𝜆 y. x) in let snd = 𝜆 p. p (𝜆 x. 𝜆 y. y) in fst ((pair (𝜆 x. x)) (𝜆 y. y))
  -->* 𝜆 x. x
# Defines church encoded pair, fst and snd, and evaluates snd (pair (fun x, x) (fun y, y))
> let pair = (fun x, fun y, fun z, z x y) in let fst = (fun p, p (fun x, fun y, x)) in let snd = (fun p, p (fun x, fun y, y)) in snd (pair (fun x, x) (fun y, y))
       let pair = 𝜆 x. 𝜆 y. 𝜆 z. (z x) y in let fst = 𝜆 p. p (𝜆 x. 𝜆 y. x) in let snd = 𝜆 p. p (𝜆 x. 𝜆 y. y) in snd ((pair (𝜆 x. x)) (𝜆 y. y))
  -->* 𝜆 y. y
For extra challenge, try adding a way to assign terms to "global" variables in the REPL.


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
