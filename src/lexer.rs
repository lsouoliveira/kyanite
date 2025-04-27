#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier,
    Keyword,
    Newline,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

// const KEYWORDS: [&str; 3] = ["def", "end"];

#[derive(Debug)]
pub struct LexerError {
    message: String,
    line: usize,
    column: usize,
}

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

fn is_newline(c: char) -> bool {
    c == '\n' || c == '\r'
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r'
}

fn is_identifier(c: char) -> bool {
    c == '_' || c.is_alphabetic() || c.is_digit(10)
}

fn is_identifier_start(c: char) -> bool {
    c == '_' || c.is_alphabetic()
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        while self.position < self.input.len() {
            let c = self.peek().unwrap();

            if is_newline(c) {
                return Ok(Some(self.read_newline()));
            }

            if is_whitespace(c) {
                self.skip_whitespace();
            }

            if is_identifier_start(c) {
                return Ok(Some(self.read_identifier()));
            }

            return Err(LexerError::new(
                format!("Unexpected character: {}", c),
                self.line,
                self.column,
            ));
        }

        Ok(None)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn peek(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn read_newline(&mut self) -> Token {
        let c = self.peek().unwrap();

        self.advance();

        self.line += 1;
        self.column = 1;

        Token {
            kind: TokenType::Newline,
            value: c.to_string(),
            line: self.line,
            column: self.column,
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();

        while let Some(c) = self.peek() {
            if is_identifier(c) {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }

        Token {
            kind: TokenType::Identifier,
            value: identifier,
            line: self.line,
            column: self.column,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && is_whitespace(self.peek().unwrap()) {
            self.advance();
        }
    }
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

impl std::error::Error for LexerError {}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid syntax: {} at line {}, column {}",
            self.message, self.line, self.column
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newline() {
        let mut lexer = Lexer::new("\n\n".to_string());

        let token = lexer.next_token().unwrap().unwrap();
        assert_eq!(token.kind, TokenType::Newline);
        assert_eq!(token.value, "\n");
        assert_eq!(token.line, 2);
        assert_eq!(token.column, 1);

        let token = lexer.next_token().unwrap().unwrap();
        assert_eq!(token.kind, TokenType::Newline);
        assert_eq!(token.value, "\n");
        assert_eq!(token.line, 3);
        assert_eq!(token.column, 1);
    }

    #[test]
    fn test_identifier() {
        let mut lexer = Lexer::new("my_function".to_string());

        let token = lexer.next_token().unwrap().unwrap();

        assert_eq!(token.kind, TokenType::Identifier);
        assert_eq!(token.value, "my_function");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);
    }

    #[test]
    fn test_identifier_starting_with_underscore() {
        let mut lexer = Lexer::new("_my_function".to_string());

        let token = lexer.next_token().unwrap().unwrap();

        assert_eq!(token.kind, TokenType::Identifier);
        assert_eq!(token.value, "_my_function");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);
    }
}
