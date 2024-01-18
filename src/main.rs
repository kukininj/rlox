mod environment;
mod error;
mod expression;
mod interpreter;
mod lox_function;
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

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::resolver::resolve;

fn run(source: String) -> Result<(), Error> {
    let tokens = scanner::scan_tokens(&source)?;
    // println!("tokens: {:#?}", tokens);
    let mut parser = Parser::new();
    let program = parser.parse(tokens)?;
    let access_table = resolve(&program)?;
    // println!("tree: {:#?}", tree);
    let mut interpreter = Interpreter::new();
    let _result = interpreter.execute(&program, access_table);
    // println!("result: {:#?}", result);

    Ok(())
}

fn print_ast(source: &String) -> Result<(), Error> {
    let tokens = scanner::scan_tokens(&source)?;
    // println!("tokens: {:#?}", tokens);
    let mut parser = Parser::new();
    let program = parser.parse(tokens)?;

    for stmt in program {
        println!("{stmt:#?}");
    }

    Ok(())
}

fn main() {
    let args: Vec<&'static mut str> = env::args().map(|arg| arg.leak()).collect();

    match args.as_slice() {
        [_] => {
            let mut line = String::new();
            print!(" >> ");
            io::stdout().flush().unwrap();
            let mut interpreter = Interpreter::new();
            let mut parser = Parser::new();

            while let Ok(_) = io::stdin().read_line(&mut line) {
                match scanner::scan_tokens(&line)
                    .and_then(|tokens| parser.parse(tokens))
                    .and_then(|program| Ok((resolve(&program)?, program)))
                    .and_then(|(access_table, program)| interpreter.execute(&program, access_table))
                {
                    Ok(_result) => {
                        // println!("{:?}", result);
                    }
                    Err(Error::ParsingError {
                        line,
                        position,
                        message: _,
                    }) => {
                        println!(
                            "Encountered error while parsing program, at line {} position {}",
                            line, position
                        );
                    }
                    Err(error) => {
                        println!("Encountered Error:");
                        println!("{:#?}", error);
                    }
                };

                print!(" >> ");
                line.clear();
                io::stdout().flush().unwrap();
            }
        }
        [_, path] if *path != "--help" => {
            let code = fs::read_to_string(path).unwrap();

            match run(code.clone()) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {:#?}", e);
                }
            }
        }
        [_, flag, path] if *flag == "--print-ast" => {
            let code = fs::read_to_string(path).unwrap();

            match print_ast(&code) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {:#?}", e);
                }
            }
        }
        _ => {
            println!("usage: rlox                              ; uruchamia repl");
            println!("       rlox [filename.lox]               ; wykonuje kod podany w pliku");
            println!("       rlox --print-ast [filename.lox]   ; wypisuje ast kodu z pliku");
        }
    }
}
