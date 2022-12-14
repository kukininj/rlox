use crate::statement::*;
use crate::{error::Error, expression::*, Token, TokenType};

struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
    line: usize,
    position: usize,
}

impl Parser {
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current_index)
    }

    fn advance(&mut self) -> Result<(), Error> {
        if self.check(&TokenType::Eof) {
            return Err(Error::ParsingError {
                line: self.line,
                position: self.position,
                message: String::from("Tried to advance after Eof"),
            });
        }
        self.current_index += 1;
        let current = self.current_token().unwrap();
        (self.line, self.position) = (current.line, current.position);
        Ok(())
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if let Some(token) = self.current_token() {
            TokenType::variant_eq(token_type, &token.token_type)
        } else {
            false
        }
    }

    fn match_token_type(&mut self, types: &[TokenType]) -> Option<Token> {
        for t in types {
            if self.check(t) {
                let t = self.current_token().unwrap().clone();
                return Some(t);
            }
        }
        None
    }

    fn consume(&mut self, t: TokenType) -> Result<(), Error> {
        if self.check(&t) {
            self.advance()?;
            Ok(())
        } else {
            Err(Error::ParsingError {
                line: self.line,
                position: self.position,
                message: format!("Expected {:?}, found: {:?}", t, self.current_token()),
            })
        }
    }

    fn statement(&mut self) -> Result<Statement, Error> {
        if self.check(&TokenType::Print) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn expression_statement(&mut self) -> Result<Statement, Error> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon).or_else(|_| {
            Err(Error::ParsingError {
                line: self.line,
                position: self.position,
                message: "Expected ';' after expression".to_string(),
            })
        })?;
        Ok(Statement::Expression(expr))
    }

    fn print_statement(&mut self) -> Result<Statement, Error> {
        self.consume(TokenType::Print)?;
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon).or_else(|_| {
            Err(Error::ParsingError {
                line: self.line,
                position: self.position,
                message: "Expected ';' after value".to_string(),
            })
        })?;
        Ok(Statement::Print(expr))
    }

    fn expression(&mut self) -> Result<Expression, Error> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression, Error> {
        let mut left = self.comparison()?;

        while let Some(operator) =
            self.match_token_type(&[TokenType::BangEqual, TokenType::EqualEqual])
        {
            self.advance()?;
            let right = self.comparison()?;
            left = Expression::from(Binary {
                left,
                operator: BinaryOperator::new(operator)?,
                right,
            });
        }

        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expression, Error> {
        let mut left = self.term()?;

        while let Some(operator) = self.match_token_type(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            self.advance()?;
            let right = self.term()?;
            left = Expression::from(Binary {
                left,
                operator: BinaryOperator::new(operator)?,
                right,
            });
        }

        Ok(left)
    }

    fn term(&mut self) -> Result<Expression, Error> {
        let mut left = self.factor()?;

        while let Some(operator) = self.match_token_type(&[TokenType::Minus, TokenType::Plus]) {
            self.advance()?;
            let right = self.factor()?;
            left = Expression::from(Binary {
                left,
                operator: BinaryOperator::new(operator)?,
                right,
            });
        }

        Ok(left)
    }

    fn factor(&mut self) -> Result<Expression, Error> {
        let mut left = self.unary()?;

        while let Some(operator) = self.match_token_type(&[TokenType::Slash, TokenType::Star]) {
            self.advance()?;
            let right = self.unary()?;
            left = Expression::from(Binary {
                left,
                operator: BinaryOperator::new(operator)?,
                right,
            });
        }

        Ok(left)
    }

    fn unary(&mut self) -> Result<Expression, Error> {
        if let Some(operator) = self.match_token_type(&[TokenType::Bang, TokenType::Minus]) {
            self.advance()?;
            let right = self.unary()?;
            Ok(Expression::from(Unary {
                operator: UnaryOperator::new(operator)?,
                right,
            }))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expression, Error> {
        if let Some(pat) = self.current_token() {
            let token = pat.clone();
            self.advance()?;
            return match token.token_type {
                TokenType::False => Ok(Expression::from(Literal {
                    value: LiteralValue::new(token)?,
                })),
                TokenType::True => Ok(Expression::from(Literal {
                    value: LiteralValue::new(token)?,
                })),
                TokenType::Nil => Ok(Expression::from(Literal {
                    value: LiteralValue::new(token)?,
                })),
                TokenType::Number(_) => Ok(Expression::from(Literal {
                    value: LiteralValue::new(token)?,
                })),
                TokenType::String(_) => Ok(Expression::from(Literal {
                    value: LiteralValue::new(token)?,
                })),
                TokenType::LeftParen => {
                    let e = self.expression()?;
                    self.consume(TokenType::RightParen)?;
                    Ok(Expression::from(Grouping { expression: e }))
                }
                _ => Err(Error::ParsingError {
                    line: self.line,
                    position: self.position,
                    message: format!("Unexpected token \"{}\", expected primary", token.lexeme),
                }),
            };
        } else {
            Err(Error::ParsingError {
                line: self.line,
                position: self.position,
                message: String::from("Expected Token at"),
            })
        }
    }

    pub fn is_at_end(self: &Self) -> bool {
        self.check(&TokenType::Eof)
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Statement>, Error> {
    let mut program = Vec::new();
    let mut parser = Parser {
        tokens,
        current_index: 0,
        line: 0,
        position: 0,
    };

    while !parser.is_at_end() {
        program.push(parser.statement()?);
    }

    Ok(program)
}

#[test]
fn test_parser() {
    macro_rules! debug_token {
        ($type:expr, $line:expr) => {
            Token {
                token_type: $type,
                lexeme: String::new(),
                line: $line,
                position: 0,
            }
        };
    }
    let tokens = vec![
        debug_token!(TokenType::LeftParen, 0),
        debug_token!(TokenType::Number(10.), 1),
        debug_token!(TokenType::Minus, 2),
        debug_token!(TokenType::Number(1.), 3),
        debug_token!(TokenType::Minus, 5),
        debug_token!(TokenType::Number(4.), 6),
        debug_token!(TokenType::RightParen, 7),
        debug_token!(TokenType::Minus, 8),
        debug_token!(TokenType::Number(4.), 9),
        debug_token!(TokenType::Semicolon, 10),
        debug_token!(TokenType::Eof, 11),
    ];

    let expr = parse(tokens).unwrap();

    let tokens = vec![
        debug_token!(TokenType::LeftParen, 0),
        debug_token!(TokenType::Number(10.), 1),
        debug_token!(TokenType::Minus, 2),
        debug_token!(TokenType::Number(1.), 3),
        debug_token!(TokenType::Minus, 4),
        debug_token!(TokenType::Number(4.), 5),
        debug_token!(TokenType::Semicolon, 6),
        debug_token!(TokenType::Eof, 7),
    ];

    let _ = parse(tokens).unwrap_err();
    println!("{:#?}", expr);
}
