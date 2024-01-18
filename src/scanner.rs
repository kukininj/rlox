use std::collections::LinkedList;

use crate::{Error, Token, TokenType};

pub fn from_slice<'a, 'b>(
    source: &'a str,
    line: &'b mut usize,
    line_position: &'b mut usize,
) -> Result<(Token, &'a str), Error> {
    let characters_skipped = skip_whitespace_characters(source, line, line_position);
    let line = *line;
    let position = *line_position;

    let source = &source[characters_skipped..];

    // rozpoznanie lekseme
    let (token_type, token_len) = match source.as_bytes() {
        [b'(', ..] => (TokenType::LeftParen, 1),
        [b')', ..] => (TokenType::RightParen, 1),
        [b'{', ..] => (TokenType::LeftBrace, 1),
        [b'}', ..] => (TokenType::RightBrace, 1),
        [b',', ..] => (TokenType::Comma, 1),
        [b'.', ..] => (TokenType::Dot, 1),
        [b'-', ..] => (TokenType::Minus, 1),
        [b'+', ..] => (TokenType::Plus, 1),
        [b';', ..] => (TokenType::Semicolon, 1),
        [b'/', ..] => (TokenType::Slash, 1),
        [b'*', ..] => (TokenType::Star, 1),
        [b'!', b'=', ..] => (TokenType::BangEqual, 2),
        [b'!', ..] => (TokenType::Bang, 1),
        [b'=', b'=', ..] => (TokenType::EqualEqual, 2),
        [b'=', ..] => (TokenType::Equal, 1),
        [b'>', b'=', ..] => (TokenType::GreaterEqual, 2),
        [b'>', ..] => (TokenType::Greater, 1),
        [b'<', b'=', ..] => (TokenType::LessEqual, 2),
        [b'<', ..] => (TokenType::Less, 1),
        [b'A'..=b'Z' | b'a'..=b'z' | b'_', ..] => {
            let s = find_identifier(source);

            if let Some(token_type) = crate::tokens::parse_keyword(s) {
                (token_type, s.len())
            } else {
                (TokenType::Identifier(String::from(s)), s.len())
            }
        }
        [b'"', ..] => {
            if let Ok(s) = find_string_literal(source) {
                // println!("s: {}", s);
                (TokenType::String(String::from(s)), s.len() + 2)
            } else {
                return Err(Error::SyntaxError {
                    line,
                    position,
                    message: String::from("Error while building a string."),
                });
            }
        }
        [b'0'..=b'9', ..] => {
            if let Ok(numeric) = find_numeric(source) {
                if let Ok(n) = numeric.parse() {
                    (TokenType::Number(n), numeric.len())
                } else {
                    println!("numeric: {}", numeric);
                    return Err(Error::SyntaxError {
                        line,
                        position,
                        message: String::from("Error while parsing a numeric"),
                    });
                }
            } else {
                return Err(Error::SyntaxError {
                    line,
                    position,
                    message: String::from("Error while building a numeric."),
                });
            }
        }
        [] => (TokenType::Eof, 0),
        _ => {
            return Err(Error::SyntaxError {
                line,
                position,
                message: format!("Unexpected character: {}", &source[0..1]),
            });
        }
    };
    *line_position += token_len;

    return Ok((
        Token {
            token_type,
            lexeme: String::from(&source[0..token_len]),
            line,
            position,
        },
        &source[token_len..],
    ));
}

fn find_numeric(source: &str) -> Result<&str, ()> {
    let mut len = 0;
    for c in source.chars() {
        match c {
            '0'..='9' => {
                len += 1;
            }
            '.' => {
                break;
            }
            _ => {
                return Ok(&source[0..len]);
            }
        }
    }
    if source.chars().nth(len + 1).unwrap_or(' ').is_digit(10) {
        // if there is a digit after '.', then continue finidng digits
        len += 1;
        for c in source[len..].chars() {
            match c {
                '0'..='9' => {
                    len += 1;
                }
                _ => {
                    return Ok(&source[0..len]);
                }
            }
        }
    }
    Ok(&source[0..len])
}

fn skip_whitespace_characters(source: &str, line: &mut usize, position: &mut usize) -> usize {
    let mut characters_skipped = 0;
    let mut handle = source;

    while handle.len() > 0 {
        match handle.as_bytes() {
            [b'/', b'/', ..] => {
                let i = handle.find('\n').unwrap_or(
                    // should only happen when there is a
                    // comment at the end of the source code
                    handle.len(),
                );
                *position = i;
                handle = &handle[i..];
                characters_skipped += i;
            }
            [b' ' | b'\r' | b'\t', ..] => {
                *position += 1;
                handle = &handle[1..];
                characters_skipped += 1;
            }
            [b'\n', ..] => {
                *line += 1;
                *position = 1;
                handle = &handle[1..];
                characters_skipped += 1;
            }
            _ => {
                break;
            }
        }
    }

    characters_skipped
}
fn find_string_literal(source: &str) -> Result<&str, ()> {
    let mut len = 0;
    for c in source.chars().skip(1) {
        match c {
            '\n' => {
                return Err(());
            }
            '"' => {
                break;
            }
            _ => {
                len += 1;
            }
        }
    }

    Ok(&source[1..=len])
}
fn find_identifier(source: &str) -> &str {
    let mut len = 0;
    while let [b'A'..=b'Z' | b'a'..=b'z' | b'_', ..] = source[len..].as_bytes() {
        len += 1;
    }

    &source[0..len]
}

pub fn scan_tokens(source: &String) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();

    let mut slice_handle = source.as_str();
    let mut line_number = 1usize;
    let mut line_position = 1usize;

    while slice_handle.len() > 0 {
        let token;
        (token, slice_handle) = from_slice(slice_handle, &mut line_number, &mut line_position)?;
        tokens.push(token);
    }

    let (line_number, line_position) = tokens
        .last()
        .map(|token| (token.line, token.position))
        .unwrap_or((1usize, 1usize));

    tokens.push(Token {
        token_type: TokenType::Eof,
        lexeme: String::from(""),
        line: line_number,
        position: line_position + 1,
    });

    return Ok(tokens);
}
