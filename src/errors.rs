#[derive(Debug, Clone)]
pub enum Error {
    RuntimeError(String),
    ParserError(String),
    UndefinedVariable(String),
    LexerError(LexerError),
    TypeError(String),
}

#[derive(Debug, Clone)]
pub struct LexerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl LexerError {
    pub fn new(message: String, line: usize, column: usize) -> Self {
        LexerError {
            message,
            line,
            column,
        }
    }
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid symbol at line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RuntimeError(msg) => write!(f, "Runtime Error: {}", msg),
            Error::UndefinedVariable(var) => write!(f, "Undefined Variable: {}", var),
            Error::ParserError(msg) => write!(f, "Parser Error: {}", msg),
            Error::LexerError(err) => write!(f, "Lexer Error: {}", err),
            Error::TypeError(msg) => write!(f, "Type Error: {}", msg),
        }
    }
}
