use crate::scanner::from_slice;
use crate::statement::{Block, Statement};
use crate::{error::Error, expression::*, Token, TokenType};

struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
    line: usize,
    position: usize,
}

macro_rules! check_m {
    ($self:ident, $token_type:pat) => {
        matches!(
            $self.current_token(),
            Some(Token {
                token_type: $token_type,
                ..
            })
        )
    };
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

    fn declaration(&mut self) -> Result<Statement, Error> {
        match self.current_token() {
            Some(Token {
                token_type: TokenType::Var,
                ..
            }) => self.variable_declaration(),
            Some(Token {
                token_type: TokenType::Fun,
                ..
            }) => self.function_declaration(),
            _ => self.statement(),
        }
    }

    fn function_declaration(&mut self) -> Result<Statement, Error> {
        self.consume(TokenType::Fun)?;

        let name = self.identifier().ok_or_else(|| Error::SyntaxError {
            line: self.line,
            position: self.position,
            message: "Expected function identifier.".to_owned(),
        })?;

        self.advance()?;

        self.consume(TokenType::LeftParen)?;

        let mut args = Vec::new();

        if !self.check(&TokenType::RightParen) {
            args.push(self.identifier().ok_or_else(|| Error::ParsingError {
                line: self.line,
                position: self.position,
                message: "Expected first argument Identifier".to_owned(),
            })?);

            self.advance()?;

            while self.check(&TokenType::Comma) {
                self.consume(TokenType::Comma)?;

                args.push(self.identifier().ok_or_else(|| Error::ParsingError {
                    line: self.line,
                    position: self.position,
                    message: "Expected argument Identifier".to_owned(),
                })?);

                self.advance()?;
            }
        }

        self.consume(TokenType::RightParen)?;

        let body = self.block_statement()?;

        Ok(Statement::Function { name, args, body })
    }

    fn variable_declaration(&mut self) -> Result<Statement, Error> {
        self.consume(TokenType::Var)?;

        let identifier = self.identifier().ok_or_else(|| Error::ParsingError {
            line: self.line,
            position: self.position,
            message: "Expected varaible Identifier".to_owned(),
        })?;

        self.advance()?;

        let mut initializer = None;
        if self.check(&TokenType::Equal) {
            self.advance()?;
            initializer = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon)?;

        Ok(Statement::Variable {
            name: identifier,
            initializer,
        })
    }

    fn statement(&mut self) -> Result<Statement, Error> {
        use TokenType as T;
        match self.current_token() {
            Some(Token {
                token_type: T::Print,
                ..
            }) => self.print_statement(),
            Some(Token {
                token_type: T::LeftBrace,
                ..
            }) => Ok(Statement::Block(self.block_statement()?)),
            Some(Token {
                token_type: T::If, ..
            }) => self.if_statement(),
            Some(Token {
                token_type: T::While,
                ..
            }) => self.while_statement(),
            Some(Token {
                token_type: T::For, ..
            }) => self.for_statement(),
            _ => self.expression_statement(),
        }
    }

    fn if_statement(&mut self) -> Result<Statement, Error> {
        self.consume(TokenType::If)?;
        let condition = self.expression()?;
        if !self.check(&TokenType::LeftBrace) {
            return Err(Error::ParsingError {
                line: self.line,
                position: self.position,
                message: "Expected the beginning of a block after an if ().".to_owned(),
            });
        }

        let then_branch = self.block_statement()?;

        let else_branch = if self.check(&TokenType::Else) {
            self.consume(TokenType::Else)?;
            Some(self.block_statement()?)
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Statement, Error> {
        self.consume(TokenType::While)?;
        let condition = self.expression()?;
        if !self.check(&TokenType::LeftBrace) {
            return Err(Error::ParsingError {
                line: self.line,
                position: self.position,
                message: "Expected the beginning of a block after an while ().".to_owned(),
            });
        }

        let body = self.block_statement()?;

        Ok(Statement::While { condition, body })
    }

    fn for_statement(&mut self) -> Result<Statement, Error> {
        self.consume(TokenType::For)?;
        self.consume(TokenType::LeftParen)?;

        let initialization = match self.current_token() {
            Some(Token {
                token_type: TokenType::Semicolon,
                ..
            }) => {
                self.consume(TokenType::Semicolon)?;
                Statement::Nop
            }
            Some(Token {
                token_type: TokenType::Var,
                ..
            }) => self.variable_declaration()?,
            _ => self.expression_statement()?,
        };

        let condition = if !self.check(&TokenType::Semicolon) {
            self.expression()?
        } else {
            Expression::Literal(Box::new(Literal {
                value: LiteralValue::True(DebugInfo {
                    lexeme: "GENERATED_VALUE".to_owned(),
                    position: self.position,
                    line: self.line,
                }),
            }))
        };

        self.consume(TokenType::Semicolon)?;

        let expression = if !self.check(&TokenType::RightParen) {
            Statement::Expression(self.expression()?)
        } else {
            Statement::Nop
        };

        self.consume(TokenType::RightParen)?;

        if !self.check(&TokenType::LeftBrace) {
            return Err(Error::ParsingError {
                line: self.line,
                position: self.position,
                message: "Expected the beginning of a block after an for (;;).".to_owned(),
            });
        }

        let mut body = self.block_statement()?;
        body.statements.push(expression);

        Ok(Statement::Block(Block {
            statements: vec![initialization, Statement::While { condition, body }],
        }))
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

    fn block_statement(&mut self) -> Result<Block, Error> {
        self.consume(TokenType::LeftBrace)?;

        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace).or_else(|_| {
            Err(Error::ParsingError {
                line: self.line,
                position: self.position,
                message: "Expected '}' after block".to_string(),
            })
        })?;

        Ok(Block { statements })
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
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, Error> {
        let expr = self.or()?;

        if self.check(&TokenType::Equal) {
            self.advance()?;
            let value = self.assignment()?;
            match expr {
                Expression::Identifier(target) => {
                    return Ok(Expression::from(Assignment {
                        target: *target,
                        value,
                    }));
                }
                _ => {
                    todo!("Assingment to non-identifier is not yet suported.")
                }
            }
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expression, Error> {
        let mut expr = self.and()?;

        while let Some(operator) = self.match_token_type(&[TokenType::Or]) {
            self.advance()?;
            let right = self.and()?;
            expr = Expression::from(Logical {
                left: expr,
                operator: LogicalOperator::new(operator)?,
                right,
            });
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expression, Error> {
        let mut expr = self.equality()?;

        while let Some(operator) = self.match_token_type(&[TokenType::And]) {
            self.advance()?;
            let right = self.equality()?;
            expr = Expression::from(Logical {
                left: expr,
                operator: LogicalOperator::new(operator)?,
                right,
            });
        }

        Ok(expr)
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
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expression, Error> {
        let calle = self.primary()?;

        if self.check(&TokenType::LeftParen) {
            let debug_info = DebugInfo {
                line: self.line,
                position: self.position,
                lexeme: "(".to_owned(),
            };
            self.consume(TokenType::LeftParen)?;

            let mut args = Vec::new();

            if !check_m!(self, TokenType::RightParen) {
                args.push(self.expression()?);

                while self.check(&TokenType::Comma) {
                    self.consume(TokenType::Comma)?;
                    args.push(self.expression()?);
                }
            }

            self.consume(TokenType::RightParen)?;

            Ok(Expression::from(Call {
                calle,
                debug_info,
                args,
            }))
        } else {
            Ok(calle)
        }
    }

    fn primary(&mut self) -> Result<Expression, Error> {
        if let Some(pat) = self.current_token() {
            let token = pat.clone();
            self.advance()?;
            return match token.token_type {
                TokenType::False
                | TokenType::True
                | TokenType::Nil
                | TokenType::Number(_)
                | TokenType::String(_) => Ok(Expression::from(Literal {
                    value: LiteralValue::new(token)?,
                })),
                TokenType::Identifier(_) => Ok(Expression::from(Identifier::new(token)?)),
                TokenType::LeftParen => {
                    let e = self.expression()?;
                    self.consume(TokenType::RightParen)?;
                    Ok(Expression::from(Grouping { expression: e }))
                }
                _ => Err(Error::ParsingError {
                    line: self.line,
                    position: self.position,
                    message: String::from("Expected Literal, Identifier or start of expression at"),
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

    pub fn synchronize(&mut self) {
        while let Ok(_) = self.advance() {
            if let Some(_) = self.match_token_type(&[
                TokenType::Class,
                TokenType::Fun,
                TokenType::Var,
                TokenType::For,
                TokenType::If,
                TokenType::While,
                TokenType::Print,
                TokenType::Return,
            ]) {
                return;
            };
            if let Ok(_) = self.consume(TokenType::Semicolon) {
                return;
            }
        }
    }

    fn identifier(&mut self) -> Option<Identifier> {
        match self.current_token() {
            Some(Token {
                token_type: TokenType::Identifier(identifier),
                lexeme,
                line,
                position,
            }) => Some(Identifier(
                identifier.clone(),
                DebugInfo {
                    line: *line,
                    position: *position,
                    lexeme: lexeme.clone(),
                },
            )),
            _ => None,
        }
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
    let mut failed = None;

    while !parser.is_at_end() {
        match parser.declaration() {
            Ok(statement) => {
                program.push(statement);
            }
            Err(error) => {
                println!("Encountered Error while parsing: {:?}", error);
                if failed.is_none() {
                    failed = Some(error);
                }
                parser.synchronize();
            }
        }
    }

    if let Some(error) = failed {
        Err(error)
    } else {
        Ok(program)
    }
}

#[test]
fn test_statements() {
    use crate::scanner;
    let expr = "
        1+1;
    ";
    let prnt = "
        print 1;
    ";
    let varb = "
        var a = 1;
    ";

    let expr = scanner::scan_tokens(&expr.to_string());
    let prnt = scanner::scan_tokens(&prnt.to_string());
    let varb = scanner::scan_tokens(&varb.to_string());

    let _expr = parse(expr.unwrap()).unwrap();
    let _prnt = parse(prnt.unwrap()).unwrap();
    let _varb = parse(varb.unwrap()).unwrap();
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

#[test]
fn test_fun_stmt() {
    use crate::scanner::scan_tokens;

    let tokens =
        scan_tokens(&"fun funkcja(arg) {print arg;}".to_owned()).expect("expected valid string");

    let fun = parse(tokens).expect("expected valid tokens comprising valid function");

    if let Some(Statement::Function { name, args, body }) = fun.get(0) {
        assert_eq!(name.0, "funkcja");
        assert_eq!(args.get(0).unwrap().0, "arg");
        match body.statements[..] {
            [Statement::Print(_)] => Ok(()),
            _ => Err(()),
        }
        .expect("invalid block");
    }
}

#[test]
fn test_call() {
    use crate::scanner::scan_tokens;

    let tokens = scan_tokens(&"funkcja(arg);".to_owned()).expect("expected valid string");

    let call = parse(tokens).expect("expected valid tokens comprising valid function");

    if let Some(Statement::Expression(expr)) = call.get(0) {
        match expr {
            Expression::Call(call) => match *call.to_owned() {
                Call {
                    calle: Expression::Identifier(identifier),
                    debug_info: _,
                    args,
                } => {
                    assert_eq!(identifier.0, "funkcja");
                    if let Expression::Identifier(_) = args.get(0).unwrap() {
                        Ok(())
                    } else {
                        Err(())
                    }
                }
                _ => Err(()),
            },
            _ => Err(()),
        }
        .expect("expected valid call in expression stmt");
    }
}
