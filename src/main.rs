mod environment;
mod error;
mod expression;
mod interpreter;
mod lox_function;
mod lox_object;
mod lox_value;
mod parser;
mod resolver;
mod scanner;
mod statement;
mod tokens;

use error::*;
use tokens::*;

use std::env;
use std::fs;
use std::io;
use std::io::Write;

fn run(source: String) -> Result<(), Error> {
    let tokens = scanner::scan_tokens(&source)?;
    // println!("tokens: {:#?}", tokens);
    let tree = parser::parse(tokens)?;
    // println!("tree: {:#?}", tree);
    let mut interpreter = interpreter::Interpreter::new();
    let result = interpreter.run(&tree);
    println!("result: {:#?}", result);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            let mut line = String::new();
            print!(" >> ");
            io::stdout().flush().unwrap();
            let mut interpreter = interpreter::Interpreter::new();

            while let Ok(_) = io::stdin().read_line(&mut line) {
                match scanner::scan_tokens(&line) {
                    Ok(tokens) => match parser::parse(tokens) {
                        Ok(tree) => {
                            let result = interpreter.run(&tree);
                            println!("{:?}", result);
                        }
                        Err(error) => {
                            println!("{:?}", error);
                        }
                    },
                    Err(error) => {
                        println!("{:?}", error);
                    }
                };

                print!(" >> ");
                line.clear();
                io::stdout().flush().unwrap();
            }
        }
        2 => {
            let code = fs::read_to_string(args.get(1).unwrap()).unwrap();

            match run(code.clone()) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
        _ => {
            println!("usage: rlox");
            println!("       rlox [filename.lox]");
        }
    }
}
