mod scanner;
mod tokens;

use tokens::*;

use std::env;
use std::fs;
use std::io;
use std::io::Write;

#[derive(Debug)]
pub enum Error {
    SyntaxError {
        line: usize,
        position: usize,
        message: String,
    },
}


fn scan_tokens(source: &String) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();

    let mut slice_handle = source.as_str();
    let mut current = 0usize;
    let mut line_number = 1usize;
    let mut line_position = 1usize;

    while slice_handle.len() > 1 {
        let (token, characters_skipped) =
            scanner::from_slice(slice_handle, &mut line_number, &mut line_position)?;
        tokens.push(token);
        current += characters_skipped;
        slice_handle = &source[current..];
    }

    return Ok(tokens);
}

fn run(source: String) -> Result<(), Error> {
    let tokens = scan_tokens(&source)?;
    println!("code: {}", source);
    println!("tokens: {:?}", tokens);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            let mut line = String::new();
            print!(" >> ");
            io::stdout().flush().unwrap();

            while let Ok(_) = io::stdin().read_line(&mut line) {
                match run(line.clone()) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
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
