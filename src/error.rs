#[derive(Debug)]
pub enum Error {
    SyntaxError {
        line: usize,
        position: usize,
        message: String,
    },
    ParsingError {
        line: usize,
        position: usize,
        message: String,
    },
    UnknownBinaryOperator {
        line: usize,
        position: usize,
        message: String,
    },
    UnknownUnaryOperator {
        line: usize,
        position: usize,
        message: String,
    },
    UnknownLiteral {
        line: usize,
        position: usize,
        message: String,
    },
    InternalRuntimeError {
        message: String,
    },
    RuntimeError {
        line: usize,
        position: usize,
        message: String,
    },
    ResolverError {
        line: usize,
        position: usize,
        message: String,
    },
}
