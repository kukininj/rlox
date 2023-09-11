use crate::*;

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Expression,
    pub operator: BinaryOperator,
    pub right: Expression,
}

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Expression,
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    String(String, DebugInfo),
    Number(f64, DebugInfo),
    True(DebugInfo),
    False(DebugInfo),
    Nil(DebugInfo),
}

impl LiteralValue {
    pub fn new(token: Token) -> Result<Self, Error> {
        match token.token_type {
            TokenType::Number(n) => Ok(Self::Number(n, DebugInfo::from(token))),
            TokenType::String(ref s) => Ok(Self::String(s.clone(), DebugInfo::from(token))),
            TokenType::True => Ok(Self::True(DebugInfo::from(token))),
            TokenType::False => Ok(Self::False(DebugInfo::from(token))),
            TokenType::Nil => Ok(Self::Nil(DebugInfo::from(token))),
            _ => Err(Error::UnknownLiteral {
                line: token.line,
                position: token.position,
                message: format!("Unknown Literal \"{:?}\".", token.lexeme),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: LiteralValue,
}

#[derive(Debug, Clone)]
pub enum LogicalOperator {
    And(DebugInfo),
    Or(DebugInfo),
}

impl LogicalOperator {
    pub fn new(token: Token) -> Result<Self, Error> {
        match token.token_type {
            TokenType::And => Ok(Self::And(DebugInfo::from(token))),
            TokenType::Or => Ok(Self::Or(DebugInfo::from(token))),
            _ => Err(Error::ParsingError {
                line: token.line,
                position: token.position,
                message: format!("Unknown logical operator \"{:?}\".", token.lexeme),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Logical {
    pub left: Expression,
    pub operator: LogicalOperator,
    pub right: Expression,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub right: Expression,
}

#[derive(Debug, Clone)]
pub struct Identifier(pub String, pub DebugInfo);

impl Identifier {
    pub fn new(token: Token) -> Result<Identifier, Error> {
        match &token.token_type {
            TokenType::Identifier(name) => Ok(Self(name.clone(), DebugInfo::from(token))),
            _ => Err(Error::ParsingError {
                line: token.line,
                position: token.position,
                message: format!("Expected identifier token, found {:?}", token),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub target: Identifier,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub calle: Expression,
    pub debug_info: DebugInfo,
    pub args: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Box<Literal>),
    Unary(Box<Unary>),
    Identifier(Box<Identifier>),
    Assignment(Box<Assignment>),
    Logical(Box<Logical>),
    Call(Box<Call>),
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

impl From<Identifier> for Expression {
    fn from(i: Identifier) -> Self {
        return Self::Identifier(Box::new(i));
    }
}

impl From<Assignment> for Expression {
    fn from(i: Assignment) -> Self {
        return Self::Assignment(Box::new(i));
    }
}

impl From<Logical> for Expression {
    fn from(i: Logical) -> Self {
        return Self::Logical(Box::new(i));
    }
}

impl From<Call> for Expression {
    fn from(i: Call) -> Self {
        return Self::Call(Box::new(i));
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
        })
        .unwrap(),
        right: e,
    });

    let grouping = Expression::from(Grouping { expression: unary });

    eprintln!("{:#?}", grouping);
}
