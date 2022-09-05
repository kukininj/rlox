use std::env;
use std::fs;
use std::io;
use std::io::Write;

#[allow(dead_code)]
#[derive(Debug)]
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
    token_type: TokenType
}

fn scan_tokens(source: &String) -> Vec<Token> {
    let tokens = Vec::new();

    return tokens;
}

fn run(source: String) {
    let tokens = scan_tokens(&source);
    println!("code: {}", source);
    println!("tokens: {:?}", tokens);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            let mut line = String::new();
            print!(" >> ");
            io::stdout().flush().unwrap();

            while let Ok(_) = io::stdin().read_line(&mut line) {
                run(line.clone());
                print!(" >> ");
                line.clear();
                io::stdout().flush().unwrap();
            }
        }
        2 => {
            let code = fs::read_to_string(args.get(1).unwrap()).unwrap();

            run(code);
        }
        _ => {
            println!("usage: rlox");
            println!("       rlox [filename.lox]");
        }
    }
}
