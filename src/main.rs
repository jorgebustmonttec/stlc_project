/*

Declarations and modules
0 / 30 points

Your task is to fill in the implementation of a programming language built on top of STLC + ℤ + 𝟚 + pairs + lists + sums + fix. Changes to terms and types are not required, as the exercise is about declarations, modules and imports.

Add pub mod module; to your lib.rs.
Copy module.rs and module from the starter.
Fill in the todo!()s in module.rs. See the function comments there for details.
The grader only tests the Declaration::to_term_fix and Module::to_term.




A new REPL is implemented in main.rs and is provided for debugging purposes. Running it with an argument, e.g. cargo run examples/std.stlc can be used import a path into the prelude of the REPL. Here is a sample from the REPL:

$ cargo run examples/std.stlc
examples/std.stlc λ reverse (cons 1 (cons 2 (nil Integer)))
[2, 1] :: [ℤ]

As an additional challenge, try writing an STLC++ module the following functions:

not :: 𝟚 → 𝟚
even :: ℤ → ℤ
len :: [ℤ] → ℤ
map :: (ℤ → ℤ) → [ℤ] → [ℤ]
zip :: [ℤ] → [ℤ] → [(ℤ → ℤ)]
unzip :: [(ℤ → ℤ)] → ([ℤ], [ℤ])
all :: (ℤ → 𝟚) → [ℤ] → 𝟚
sums :: [ℤ + ℤ] → (ℤ, ℤ)
However, no points are awarded for completing this challenge.

*/

use stlc_project::{
    module::{parse::parse_module, Module},
    term::parse::parse_term,
};
use nom::combinator::all_consuming;
use nom::Parser;

use std::{
    env::{args, current_dir},
    path::Path,
};

use rustyline::{error::ReadlineError, DefaultEditor};

fn process(
    file: Option<&str>,
    line: &str,
    module: Module,
) -> Result<(), Box<dyn std::error::Error>> {
    let (_, body) = all_consuming(parse_term)
        .parse(line)
        .map_err(|e| e.to_string())?;
    let basepath = if let Some(p) = file {
        current_dir()?.join(Path::new(p).parent().expect("import to have a parent"))
    } else {
        current_dir()?
    };
    let t = module.to_term(basepath, body)?;
    println!("{1} :: {0}", t.type_check()?, t.multistep());
    Ok(())
}

fn prompt<T: AsRef<str>>(file: Option<T>) -> String {
    match file {
        None => "λ ".to_string(),
        Some(name) => format!("{} λ ", name.as_ref()),
    }
}

fn start_repl(file: Option<&str>, module: Module) -> Result<(), Box<dyn std::error::Error>> {
    let p = prompt(file);
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline(&p);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                if line.trim() == ":q" {
                    break;
                }

                if let Err(e) = process(file, &line, module.clone()) {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match args().nth(1) {
        Some(name) => {
            let code = std::fs::read_to_string(&name).expect("failed to import file");
            let m = all_consuming(parse_module)
                .parse(&code)
                .expect("failed to parse code")
                .1;
            start_repl(Some(&name), m)?;
        }
        None => start_repl(None, Module::new())?,
    };
    Ok(())
}
