use colored::Colorize;

#[derive(Debug, Clone)]
pub enum Error {
    RuntimeError(String),
    ParserError(String),
    UndefinedVariable(String),
    LexerError(LexerError),
    TypeError(String),
    ValueError(String),
    BreakInterrupt(String),
    NotImplemented(String),
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
            Error::RuntimeError(msg) => write!(f, "{}", format_error("Runtime Error", msg)),
            Error::ParserError(msg) => write!(f, "{}", format_error("Parser Error", msg)),
            Error::UndefinedVariable(var) => write!(
                f,
                "{}: {}",
                "Undefined Variable".purple().bold(),
                var.red().bold()
            ),
            Error::LexerError(lexer_error) => write!(f, "{}", lexer_error),
            Error::TypeError(msg) => write!(f, "{}", format_error("Type Error", msg)),
            Error::ValueError(msg) => write!(f, "{}", format_error("Value Error", msg)),
            Error::BreakInterrupt(msg) => write!(
                f,
                "{}: {}",
                "Break Interrupt".purple().bold(),
                msg.red().bold()
            ),
            Error::NotImplemented(msg) => write!(
                f,
                "{}: {}",
                "Not Implemented".purple().bold(),
                msg.red().bold()
            ),
        }
    }
}

fn format_error(error_type: &str, message: &str) -> String {
    format!("{}: {}", error_type.purple().bold(), message.purple())
}
