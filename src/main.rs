use std::env;
use std::fs;
use std::io;
use std::io::Write;

#[derive(Debug)]
enum Error {
    SyntaxError { line: usize, position: usize },
}

#[allow(dead_code)]
#[derive(Debug)]
#[rustfmt::skip]
enum TokenType {
  // Single-character tokens.
  Leftparen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  // One or two character tokens.
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  // Literals.
  Identifier, String, Number,

  // Keywords.
  And, Class, Else, False, Fun, For, If, Nil, Or,
  Print, Return, Super, This, True, Var, While,

  Eof
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
}

fn scan_tokens(source: &String) -> Result<Vec<Token>, Error> {
    let tokens = Vec::new();

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
