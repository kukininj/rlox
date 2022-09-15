use crate::*;

#[derive(Debug)]
pub struct DebugInfo {
    pub line: usize,
    pub position: usize,
    pub lexeme: String,
}

impl From<Token> for DebugInfo {
    fn from(token: Token) -> Self {
        Self {
            line: token.line,
            position: token.position,
            lexeme: token.lexeme,
        }
    }
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add(DebugInfo),
    Subtract(DebugInfo),
    Multiply(DebugInfo),
    Divide(DebugInfo),
    Equal(DebugInfo),
    NotEqual(DebugInfo),
    Less(DebugInfo),
    LessEqual(DebugInfo),
    Greater(DebugInfo),
    GreaterEqual(DebugInfo),
}

impl BinaryOperator {
    pub fn new(token: Token) -> Result<Self, Error> {
        match token.token_type {
            TokenType::Plus => Ok(Self::Add(DebugInfo::from(token))),
            TokenType::Minus => Ok(Self::Subtract(DebugInfo::from(token))),
            TokenType::Slash => Ok(Self::Divide(DebugInfo::from(token))),
            TokenType::Star => Ok(Self::Multiply(DebugInfo::from(token))),
            TokenType::BangEqual => Ok(Self::NotEqual(DebugInfo::from(token))),
            TokenType::EqualEqual => Ok(Self::Equal(DebugInfo::from(token))),
            TokenType::Greater => Ok(Self::Greater(DebugInfo::from(token))),
            TokenType::GreaterEqual => Ok(Self::GreaterEqual(DebugInfo::from(token))),
            TokenType::Less => Ok(Self::Less(DebugInfo::from(token))),
            TokenType::LessEqual => Ok(Self::LessEqual(DebugInfo::from(token))),
            _ => Err(Error::UnknownBinaryOperator {
                line: token.line,
                position: token.position,
                message: format!("Unknown Binary Operator \"{:?}\".", token.lexeme),
            }),
        }
    }
}

#[derive(Debug)]
pub struct Binary {
    pub left: Expression,
    pub operator: BinaryOperator,
    pub right: Expression,
}

#[derive(Debug)]
pub struct Grouping {
    pub expression: Expression,
}

#[derive(Debug)]
pub enum LiteralValue {
    String(String, DebugInfo),
    Number(f64, DebugInfo),
    True(DebugInfo),
    False(DebugInfo),
}

impl LiteralValue {
    pub fn new(token: Token) -> Result<Self, Error> {
        match token.token_type {
            TokenType::Number(n) => Ok(Self::Number(n, DebugInfo::from(token))),
            TokenType::String(ref s) => Ok(Self::String(s.clone(), DebugInfo::from(token))),
            TokenType::True => Ok(Self::True(DebugInfo::from(token))),
            TokenType::False => Ok(Self::False(DebugInfo::from(token))),
            _ => Err(Error::UnknownLiteral {
                line: token.line,
                position: token.position,
                message: format!("Unknown Literal \"{:?}\".", token.lexeme),
            }),
        }
    }
}

#[derive(Debug)]
pub struct Literal {
    pub value: LiteralValue,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Not(DebugInfo),
    Negative(DebugInfo),
}

impl UnaryOperator {
    pub fn new(token: Token) -> Result<Self, Error> {
        match token.token_type {
            TokenType::Bang => Ok(Self::Not(DebugInfo::from(token))),
            TokenType::Minus => Ok(Self::Negative(DebugInfo::from(token))),
            _ => Err(Error::UnknownUnaryOperator {
                line: token.line,
                position: token.position,
                message: format!("Unknown Unary Operator \"{:?}\".", token.lexeme),
            }),
        }
    }
}

#[derive(Debug)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub right: Expression,
}

#[derive(Debug)]
pub enum Expression {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Box<Literal>),
    Unary(Box<Unary>),
}

impl From<Binary> for Expression {
    fn from(g: Binary) -> Self {
        return Self::Binary(Box::new(g));
    }
}

impl From<Grouping> for Expression {
    fn from(g: Grouping) -> Self {
        return Self::Grouping(Box::new(g));
    }
}

impl From<Literal> for Expression {
    fn from(g: Literal) -> Self {
        return Self::Literal(Box::new(g));
    }
}

impl From<Unary> for Expression {
    fn from(g: Unary) -> Self {
        return Self::Unary(Box::new(g));
    }
}

#[test]
fn expression_test() {
    let e = Expression::from(Binary {
        operator: BinaryOperator::new(Token {
            token_type: TokenType::Minus,
            lexeme: String::new(),
            line: 0,
            position: 0,
        })
        .unwrap(),
        left: Expression::from(Literal {
            value: LiteralValue::new(Token {
                token_type: TokenType::Number(10.),
                lexeme: String::new(),
                line: 0,
                position: 0,
            })
            .unwrap(),
        }),
        right: Expression::from(Literal {
            value: LiteralValue::new(Token {
                token_type: TokenType::Number(10.),
                lexeme: String::new(),
                line: 0,
                position: 0,
            })
            .unwrap(),
        }),
    });

    let unary = Expression::from(Unary {
        operator: UnaryOperator::new(Token {
            token_type: TokenType::Minus,
            lexeme: String::new(),
            line: 0,
            position: 0,
        }).unwrap(),
        right: e,
    });

    let grouping = Expression::from(Grouping { expression: unary });

    println!("{:#?}", grouping);
}
