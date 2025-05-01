use crate::errors::{Error, LexerError};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier,
    Newline,
    StringLiteral,
    LeftParen,
    RightParen,
    Equal,
    NumberLiteral,
    Def,
    End,
    Comma
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
    symbols: HashMap<String, TokenType>,
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

fn is_string_literal(c: char) -> bool {
    c == '"' || c == '\''
}

fn is_number_literal(c: char) -> bool {
    c.is_digit(10) || c == '+' || c == '-'
}

fn is_keyword(identifier: &str) -> bool {
    symbols().contains_key(identifier)
}

// TODO: Replace with a static map
fn symbols() -> HashMap<String, TokenType> {
    let mut symbols = HashMap::new();
    symbols.insert("(".to_string(), TokenType::LeftParen);
    symbols.insert(")".to_string(), TokenType::RightParen);
    symbols.insert("=".to_string(), TokenType::Equal);
    symbols.insert("def".to_string(), TokenType::Def);
    symbols.insert("end".to_string(), TokenType::End);
    symbols.insert(",".to_string(), TokenType::Comma);
    symbols
}

fn is_symbol(c: char) -> bool {
    symbols().contains_key(&c.to_string())
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            position: 0,
            line: 1,
            column: 1,
            symbols: symbols(),
        }
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, Error> {
        while self.position < self.input.len() {
            let c = self.peek().unwrap();

            if is_whitespace(c) {
                self.skip_whitespace();
                continue;
            }

            if is_newline(c) {
                return Ok(Some(self.read_newline()));
            }

            if is_symbol(c) {
                return Ok(Some(self.read_symbol()));
            }

            if is_string_literal(c) {
                return self.read_string_literal();
            }

            if is_number_literal(c) {
                return self.read_number_literal();
            }

            if is_identifier_start(c) {
                return Ok(Some(self.read_identifier()));
            }

            return Err(Error::LexerError(LexerError::new(
                format!("Invalid symbol: {}", c),
                self.line,
                self.column,
            )));
        }

        Ok(None)
    }

    fn advance(&mut self) {
        self.position += 1;
        self.column += 1;
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

    fn read_symbol(&mut self) -> Token {
        let c = self.peek().unwrap();
        let kind = self.symbols.get(&c.to_string()).unwrap().clone();
        let column_start = self.column;

        self.advance();

        Token {
            kind,
            value: c.to_string(),
            line: self.line,
            column: column_start,
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        let column_start = self.column;

        while let Some(c) = self.peek() {
            if is_identifier(c) {
                identifier.push(c);

                if is_keyword(&identifier) {
                    self.advance();

                    let kind = self.symbols.get(&identifier).unwrap().clone();

                    return Token {
                        kind,
                        value: identifier,
                        line: self.line,
                        column: column_start,
                    };
                }

                self.advance();
            } else {
                break;
            }
        }

        Token {
            kind: TokenType::Identifier,
            value: identifier,
            line: self.line,
            column: column_start,
        }
    }

    fn read_string_literal(&mut self) -> Result<Option<Token>, Error> {
        let mut content = String::new();
        let quote_character = self.peek().unwrap();
        let mut is_terminated = false;
        let column_start = self.column;

        self.advance();

        while let Some(c) = self.peek() {
            if c == quote_character {
                is_terminated = true;
                self.advance();
                break;
            } else {
                content.push(c);
                self.advance();
            }
        }

        if !is_terminated {
            return Err(Error::LexerError(LexerError::new(
                "Unterminated string literal".to_string(),
                self.line,
                column_start,
            )));
        }

        Ok(Some(Token {
            kind: TokenType::StringLiteral,
            value: content,
            line: self.line,
            column: column_start,
        }))
    }

    fn read_number_literal(&mut self) -> Result<Option<Token>, Error> {
        let mut number = String::new();
        let column_start = self.column;
        let mut dot_seen = false;
        let mut number_seen = false;

        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                number_seen = true;
                number.push(c);
                self.advance();
            } else if c == '+' || c == '-' && !number_seen {
                number.push(c);
                self.advance();
            } else if c == '.' && !dot_seen {
                dot_seen = true;
                number.push(c);
                self.advance();
            } else if c == '.' && dot_seen {
                return Err(Error::LexerError(LexerError::new(
                    "Invalid number literal".to_string(),
                    self.line,
                    column_start,
                )));
            } else {
                break;
            }
        }

        Ok(Some(Token {
            kind: TokenType::NumberLiteral,
            value: number,
            line: self.line,
            column: column_start,
        }))
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && is_whitespace(self.peek().unwrap()) {
            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skips_whitespace() {
        let mut lexer = Lexer::new("   \t\n".to_string());

        let token = lexer.next_token().unwrap().unwrap();
        assert_eq!(token.kind, TokenType::Newline);
        assert_eq!(token.value, "\n");
        assert_eq!(token.line, 2);
        assert_eq!(token.column, 1);
    }

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

    #[test]
    fn test_string_literal_with_quotes() {
        let mut lexer = Lexer::new("\"my string\"".to_string());

        let token = lexer.next_token().unwrap().unwrap();

        assert_eq!(token.kind, TokenType::StringLiteral);
        assert_eq!(token.value, "my string");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);
    }

    #[test]
    fn test_string_literal_with_single_quotes() {
        let mut lexer = Lexer::new("'my string'".to_string());

        let token = lexer.next_token().unwrap().unwrap();

        assert_eq!(token.kind, TokenType::StringLiteral);
        assert_eq!(token.value, "my string");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);
    }

    #[test]
    fn test_unterminated_string_literal() {
        let mut lexer = Lexer::new("\"my string".to_string());

        let result = lexer.next_token();

        assert!(result.is_err());
        let error = result.unwrap_err();
        let lexer_error = match error {
            Error::LexerError(err) => err,
            _ => panic!("Expected LexerError"),
        };
        assert_eq!(lexer_error.message, "Unterminated string literal");
    }
    #[test]
    fn test_symbols() {
        for symbol in symbols().keys() {
            let mut lexer = Lexer::new(symbol.clone());
            let token = lexer.next_token().unwrap().unwrap();
            assert_eq!(&token.kind, symbols().get(symbol).unwrap());
            assert_eq!(&token.value, symbol);
            assert_eq!(token.line, 1);
            assert_eq!(token.column, 1);
        }
    }

    #[test]
    fn test_number_literal_unsigned() {
        let mut lexer = Lexer::new("12345".to_string());

        let token = lexer.next_token().unwrap().unwrap();

        assert_eq!(token.kind, TokenType::NumberLiteral);
        assert_eq!(token.value, "12345");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);
    }

    #[test]
    fn test_number_literal_signed() {
        let mut lexer = Lexer::new("-12345".to_string());

        let token = lexer.next_token().unwrap().unwrap();

        assert_eq!(token.kind, TokenType::NumberLiteral);
        assert_eq!(token.value, "-12345");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);
    }

    #[test]
    fn test_number_literal_with_decimal() {
        let mut lexer = Lexer::new("123.45".to_string());

        let token = lexer.next_token().unwrap().unwrap();

        assert_eq!(token.kind, TokenType::NumberLiteral);
        assert_eq!(token.value, "123.45");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);
    }

    #[test]
    fn test_number_literal_with_plus() {
        let mut lexer = Lexer::new("+12345".to_string());

        let token = lexer.next_token().unwrap().unwrap();

        assert_eq!(token.kind, TokenType::NumberLiteral);
        assert_eq!(token.value, "+12345");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);
    }

    fn test_number_literal_with_extra_dot() {
        let mut lexer = Lexer::new("12345.0.".to_string());

        let token = lexer.next_token();

        assert!(token.is_err());
    }

    fn test_def_keyword() {
        let mut lexer = Lexer::new("def my_method\nend\n".to_string());
        let tokens = [
            lexer.next_token().unwrap().unwrap(),
            lexer.next_token().unwrap().unwrap(),
        ];

        assert_eq!(tokens[0].kind, TokenType::Def);
        assert_eq!(tokens[0].value, "def");
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 1);

        assert_eq!(tokens[1].kind, TokenType::Identifier);
        assert_eq!(tokens[1].value, "my_method");
        assert_eq!(tokens[1].line, 1);
        assert_eq!(tokens[1].column, 5);
    }

    fn test_end_keyword() {
        let mut lexer = Lexer::new("end".to_string());

        let token = lexer.next_token().unwrap().unwrap();

        assert_eq!(token.kind, TokenType::End);
        assert_eq!(token.value, "end");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);
    }

    fn test_comma() {
        let mut lexer = Lexer::new(",".to_string());

        let token = lexer.next_token().unwrap().unwrap();

        assert_eq!(token.kind, TokenType::Comma);
        assert_eq!(token.value, ",");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);
    }
}
