use crate::ast;
use crate::errors::Error;
use crate::lexer::Lexer;
use crate::lexer::{Token, TokenType};

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser {
            lexer,
            current_token: None,
        }
    }

    pub fn parse(&mut self) -> Result<ast::ASTNode, Error> {
        self.next_token().unwrap();

        let mut statements = Vec::new();

        while self.current_token.is_some() {
            self.skip_newlines();
            match self.parse_statement() {
                Ok(statement) => statements.push(statement),
                Err(e) => return Err(e),
            }
        }

        Ok(ast::ASTNode::Module(ast::Module::new(statements)))
    }

    fn parse_statement(&mut self) -> Result<Box<ast::ASTNode>, Error> {
        let expr = self.parse_expression()?;

        self.expect(TokenType::Newline)?;
        self.skip_newlines();

        Ok(expr)
    }

    fn parse_expression(&mut self) -> Result<Box<ast::ASTNode>, Error> {
        if let Some(token) = self.accept(TokenType::Identifier) {
            let identifier = Box::new(ast::ASTNode::Identifier(ast::Identifier {
                name: token.value.clone(),
            }));

            if let Some(_) = self.accept(TokenType::LeftParen) {
                let mut arguments = Vec::new();

                if self.accept(TokenType::RightParen).is_none() {
                    arguments.push(self.parse_expression()?);
                    self.expect(TokenType::RightParen)?;
                }

                return Ok(Box::new(ast::ASTNode::MethodCall(ast::MethodCall::new(
                    identifier, arguments,
                ))));
            } else if self.accept(TokenType::Equal).is_some() {
                let value = self.parse_expression()?;
                return Ok(Box::new(ast::ASTNode::Assignment(ast::Assignment::new(
                    token.value.clone(),
                    value,
                ))));
            }

            return Ok(identifier);
        } else if let Some(token) = self.accept(TokenType::StringLiteral) {
            return Ok(Box::new(ast::ASTNode::StringLiteral(token.value.clone())));
        } else if let Some(token) = self.accept(TokenType::NumberLiteral) {
            return Ok(Box::new(ast::ASTNode::NumberLiteral(
                token.value.parse::<f64>().map_err(|_| {
                    Error::ParserError(format!(
                        "Invalid number literal: {} at line {}, column {}",
                        token.value, token.line, token.column
                    ))
                })?,
            )));
        }

        let token = self.peek().unwrap();

        Err(Error::ParserError(format!(
            "Unexpected token {} at line {}, column {}",
            token.value, token.line, token.column
        )))
    }

    fn peek(&self) -> Option<&Token> {
        self.current_token.as_ref()
    }

    fn accept(&mut self, token_type: TokenType) -> Option<Token> {
        if let Some(ref token) = self.current_token {
            if token.kind == token_type {
                let token = self.current_token.clone();
                self.next_token().unwrap();
                return token;
            }
        }

        None
    }

    fn expect(&mut self, token_type: TokenType) -> Result<Token, Error> {
        if let Some(ref token) = self.current_token {
            if token.kind == token_type {
                let token = self.current_token.clone();
                self.next_token().unwrap();
                return Ok(token.unwrap());
            } else {
                return Err(Error::ParserError(format!(
                    "Expected token \"{}\" at line {}, column {}",
                    token.value, token.line, token.column
                )));
            }
        }

        Err(Error::ParserError(format!("Unexpected token",)))
    }

    fn next_token(&mut self) -> Result<(), Error> {
        self.current_token = self.lexer.next_token()?;

        Ok(())
    }

    fn skip_newlines(&mut self) {
        while self.accept(TokenType::Newline).is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_empty_module() {
        let input = "";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();
        assert_eq!(result, ast::ASTNode::Module(ast::Module::new(vec![])));
    }

    #[test]
    fn test_parse_single_statement() {
        let input = "identifier\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![Box::new(
            ast::ASTNode::Identifier(ast::Identifier {
                name: "identifier".to_string(),
            }),
        )]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_multiple_statements() {
        let input = "identifier1\nidentifier2\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![
            Box::new(ast::ASTNode::Identifier(ast::Identifier {
                name: "identifier1".to_string(),
            })),
            Box::new(ast::ASTNode::Identifier(ast::Identifier {
                name: "identifier2".to_string(),
            })),
        ]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_invalid_token() {
        let input = "identifier1\n(\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse();

        assert!(result.is_err());
        if let Err(Error::ParserError(msg)) = result {
            assert!(msg.contains("Unexpected token"));
        } else {
            panic!("Expected a ParserError");
        }
    }

    #[test]
    fn test_parse_identiifer() {
        let input = "my_function\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![Box::new(
            ast::ASTNode::Identifier(ast::Identifier {
                name: "my_function".to_string(),
            }),
        )]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_string_literal() {
        let input = "\"my string\"\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![Box::new(
            ast::ASTNode::StringLiteral("my string".to_string()),
        )]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_string_literal_with_single_quotes() {
        let input = "'my string'\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![Box::new(
            ast::ASTNode::StringLiteral("my string".to_string()),
        )]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_method_call() {
        let input = "my_function()\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![Box::new(
            ast::ASTNode::MethodCall(ast::MethodCall {
                name: Box::new(ast::ASTNode::Identifier(ast::Identifier {
                    name: "my_function".to_string(),
                })),
                arguments: vec![],
            }),
        )]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_assignment() {
        let input = "my_variable = \"42\"\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![Box::new(
            ast::ASTNode::Assignment(ast::Assignment {
                name: "my_variable".to_string(),
                value: Box::new(ast::ASTNode::StringLiteral("42".to_string())),
            }),
        )]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_number_literal() {
        let input = "42\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![Box::new(
            ast::ASTNode::NumberLiteral(42.0),
        )]));

        assert_eq!(result, expected);
    }
}
