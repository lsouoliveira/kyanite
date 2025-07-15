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
            match self.parse_statement() {
                Ok(statement) => statements.push(statement),
                Err(e) => return Err(e),
            }
        }

        Ok(ast::ASTNode::Module(ast::Module::new(statements)))
    }

    fn parse_statement(&mut self) -> Result<Box<ast::ASTNode>, Error> {
        self.skip_newlines();

        let stmt = if self.accept(TokenType::Def).is_some() {
            self.parse_method_def()?
        } else if self.accept(TokenType::Class).is_some() {
            self.parse_class_def()?
        } else if self.accept(TokenType::If).is_some() {
            self.parse_if_statement()?
        } else if self.accept(TokenType::Import).is_some() {
            self.parse_import()?
        } else {
            self.parse_expression()?
        };

        if self.peek().is_none() {
            return Ok(stmt);
        }

        self.expect(TokenType::Newline)?;
        self.skip_newlines();

        Ok(stmt)
    }

    fn parse_class_def(&mut self) -> Result<Box<ast::ASTNode>, Error> {
        let identifier = self.expect(TokenType::Identifier)?;

        let mut body = Vec::new();

        self.expect(TokenType::Newline)?;

        while self.peek().is_some() {
            if let Some(_) = self.accept(TokenType::End) {
                break;
            }

            match self.parse_statement() {
                Ok(statement) => {
                    body.push(statement);
                }
                Err(e) => return Err(e),
            }
        }

        let class_def = ast::ClassDef::new(identifier.value.clone(), body);

        Ok(Box::new(ast::ASTNode::ClassDef(class_def)))
    }

    fn parse_if_statement(&mut self) -> Result<Box<ast::ASTNode>, Error> {
        let test = self.parse_expression()?;

        self.expect(TokenType::Newline)?;

        let mut body = Vec::new();

        while self.peek().is_some() {
            if let Some(_) = self.accept(TokenType::End) {
                break;
            }

            match self.parse_statement() {
                Ok(statement) => {
                    body.push(statement);
                }
                Err(e) => return Err(e),
            }
        }

        let if_node = ast::If::new(test, body);

        Ok(Box::new(ast::ASTNode::If(if_node)))
    }

    fn parse_import(&mut self) -> Result<Box<ast::ASTNode>, Error> {
        let mut module_name = String::new();

        while self.peek().is_some() && self.peek().unwrap().kind != TokenType::Newline {
            module_name.push_str(&self.peek().unwrap().value);
            self.next_token().unwrap();
        }

        Ok(Box::new(ast::ASTNode::Import(ast::Import {
            name: module_name,
        })))
    }

    fn parse_method_def(&mut self) -> Result<Box<ast::ASTNode>, Error> {
        let mut parameters = Vec::new();
        let mut body = Vec::new();
        let identifier = self.expect(TokenType::Identifier)?;

        if let Some(_) = self.accept(TokenType::LeftParen) {
            parameters = self.parse_parameters()?;
            self.expect(TokenType::RightParen)?;
        }

        self.expect(TokenType::Newline)?;

        while self.peek().is_some() {
            if let Some(_) = self.accept(TokenType::End) {
                break;
            }

            match self.parse_statement() {
                Ok(statement) => {
                    body.push(statement);
                }
                Err(e) => return Err(e),
            }
        }

        let method_def = ast::MethodDef::new(identifier.value.clone(), parameters, body);

        Ok(Box::new(ast::ASTNode::MethodDef(method_def)))
    }

    fn parse_parameters(&mut self) -> Result<Vec<Box<ast::ASTNode>>, Error> {
        let mut parameters = Vec::new();

        while let Some(token) = self.accept(TokenType::Identifier) {
            parameters.push(Box::new(ast::ASTNode::Identifier(ast::Identifier {
                name: token.value.clone(),
            })));

            if self.accept(TokenType::Comma).is_none() {
                break;
            }
        }

        Ok(parameters)
    }

    fn parse_expression(&mut self) -> Result<Box<ast::ASTNode>, Error> {
        Ok(self.parse_comparison()?)
    }

    fn parse_comparison(&mut self) -> Result<Box<ast::ASTNode>, Error> {
        let mut primary = self.parse_sum()?;

        loop {
            if self.accept(TokenType::EqEqual).is_some() {
                let right = self.parse_sum()?;

                primary = Box::new(ast::ASTNode::Compare(ast::Compare {
                    left: primary,
                    operator: ast::Operator::Equal,
                    right,
                }));
            } else {
                break;
            }
        }

        Ok(primary)
    }

    fn parse_sum(&mut self) -> Result<Box<ast::ASTNode>, Error> {
        let mut primary = self.parse_primary()?;
        let operators = [TokenType::Plus];

        loop {
            let mut check = false;

            for operator in &operators {
                if let Some(_) = self.accept(operator.clone()) {
                    let right = self.parse_primary()?;
                    primary = Box::new(ast::ASTNode::BinOp(ast::BinOp {
                        left: primary,
                        operator: operator.clone(),
                        right,
                    }));
                    check = true;
                }
            }

            if !check {
                break;
            }
        }

        Ok(primary)
    }

    fn parse_primary(&mut self) -> Result<Box<ast::ASTNode>, Error> {
        let mut primary = self.parse_atom()?;

        loop {
            if self.accept(TokenType::LeftParen).is_some() {
                let mut arguments = Vec::new();

                while self.peek().is_some() && self.peek().unwrap().kind != TokenType::RightParen {
                    arguments.push(self.parse_expression()?);

                    if self.accept(TokenType::Comma).is_none() {
                        break;
                    }
                }

                self.expect(TokenType::RightParen)?;

                primary = Box::new(ast::ASTNode::MethodCall(ast::MethodCall::new(
                    primary, arguments,
                )));
            } else if self.accept(TokenType::Equal).is_some() {
                let value = self.parse_expression()?;
                primary = Box::new(ast::ASTNode::Assignment(ast::Assignment::new(
                    primary, value,
                )));
            } else if self.accept(TokenType::Dot).is_some() {
                let identifier = self.expect(TokenType::Identifier)?;

                primary = Box::new(ast::ASTNode::Attribute(ast::Attribute::new(
                    primary,
                    identifier.value.clone(),
                )));
            } else {
                break;
            }
        }

        Ok(primary)
    }

    fn parse_atom(&mut self) -> Result<Box<ast::ASTNode>, Error> {
        if let Some(token) = self.accept(TokenType::Identifier) {
            return Ok(Box::new(ast::ASTNode::Identifier(ast::Identifier {
                name: token.value.clone(),
            })));
        }

        if let Some(token) = self.accept(TokenType::StringLiteral) {
            return Ok(Box::new(ast::ASTNode::StringLiteral(token.value.clone())));
        }

        if let Some(token) = self.accept(TokenType::Plus) {
            let operand = self.parse_atom()?;

            return Ok(Box::new(ast::ASTNode::UnaryOp(ast::UnaryOp {
                operator: token.kind,
                operand,
            })));
        }

        if let Some(token) = self.accept(TokenType::Minus) {
            let operand = self.parse_atom()?;

            return Ok(Box::new(ast::ASTNode::UnaryOp(ast::UnaryOp {
                operator: token.kind,
                operand,
            })));
        }

        if let Some(token) = self.accept(TokenType::NumberLiteral) {
            return Ok(Box::new(ast::ASTNode::NumberLiteral(
                token.value.parse::<f64>().map_err(|_| {
                    Error::ParserError(format!(
                        "Invalid number literal: {} at line {}, column {}",
                        token.value, token.line, token.column
                    ))
                })?,
            )));
        }

        Err(Error::ParserError(format!(
            "Unexpected token {} at line {}, column {}",
            self.peek().unwrap().value,
            self.peek().unwrap().line,
            self.peek().unwrap().column
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
                name: Box::new(ast::ASTNode::Identifier(ast::Identifier {
                    name: "my_variable".to_string(),
                })),
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

    #[test]
    fn test_parse_method_def_without_parens() {
        let input = "def my_method\nend\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![Box::new(
            ast::ASTNode::MethodDef(ast::MethodDef {
                name: "my_method".to_string(),
                parameters: vec![],
                body: vec![],
            }),
        )]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_method_def_with_parens() {
        let input = "def my_method()\nend\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![Box::new(
            ast::ASTNode::MethodDef(ast::MethodDef {
                name: "my_method".to_string(),
                parameters: vec![],
                body: vec![],
            }),
        )]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_method_def_with_body() {
        let input = "def my_method\nmy_variable = \"42\"\nend\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![Box::new(
            ast::ASTNode::MethodDef(ast::MethodDef {
                name: "my_method".to_string(),
                parameters: vec![],
                body: vec![Box::new(ast::ASTNode::Assignment(ast::Assignment {
                    name: Box::new(ast::ASTNode::Identifier(ast::Identifier {
                        name: "my_variable".to_string(),
                    })),
                    value: Box::new(ast::ASTNode::StringLiteral("42".to_string())),
                }))],
            }),
        )]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_method_def_with_parameters() {
        let input = "def my_method(param1, param2)\nend\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![Box::new(
            ast::ASTNode::MethodDef(ast::MethodDef {
                name: "my_method".to_string(),
                parameters: vec![
                    Box::new(ast::ASTNode::Identifier(ast::Identifier {
                        name: "param1".to_string(),
                    })),
                    Box::new(ast::ASTNode::Identifier(ast::Identifier {
                        name: "param2".to_string(),
                    })),
                ],
                body: vec![],
            }),
        )]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_class_def() {
        let input = "class MyClass\nend\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![Box::new(
            ast::ASTNode::ClassDef(ast::ClassDef {
                name: "MyClass".to_string(),
                body: vec![],
            }),
        )]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_attribute() {
        let input = "my_object.my_attribute\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![Box::new(
            ast::ASTNode::Attribute(ast::Attribute {
                name: Box::new(ast::ASTNode::Identifier(ast::Identifier {
                    name: "my_object".to_string(),
                })),
                value: "my_attribute".to_string(),
            }),
        )]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_instance_method_call() {
        let input = "my_object.my_attribute()\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![Box::new(
            ast::ASTNode::MethodCall(ast::MethodCall {
                name: Box::new(ast::ASTNode::Attribute(ast::Attribute {
                    name: Box::new(ast::ASTNode::Identifier(ast::Identifier {
                        name: "my_object".to_string(),
                    })),
                    value: "my_attribute".to_string(),
                })),
                arguments: vec![],
            }),
        )]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_equal_operator() {
        let input = "a == b\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![Box::new(
            ast::ASTNode::Compare(ast::Compare {
                left: Box::new(ast::ASTNode::Identifier(ast::Identifier {
                    name: "a".to_string(),
                })),
                operator: ast::Operator::Equal,
                right: Box::new(ast::ASTNode::Identifier(ast::Identifier {
                    name: "b".to_string(),
                })),
            }),
        )]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_equal_operators() {
        let input = "a == b == c\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![Box::new(
            ast::ASTNode::Compare(ast::Compare {
                left: Box::new(ast::ASTNode::Compare(ast::Compare {
                    left: Box::new(ast::ASTNode::Identifier(ast::Identifier {
                        name: "a".to_string(),
                    })),
                    operator: ast::Operator::Equal,
                    right: Box::new(ast::ASTNode::Identifier(ast::Identifier {
                        name: "b".to_string(),
                    })),
                })),
                operator: ast::Operator::Equal,
                right: Box::new(ast::ASTNode::Identifier(ast::Identifier {
                    name: "c".to_string(),
                })),
            }),
        )]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_if_statement() {
        let input = "if condition\nmy_variable = \"42\"\nend\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![Box::new(ast::ASTNode::If(
            ast::If {
                test: Box::new(ast::ASTNode::Identifier(ast::Identifier {
                    name: "condition".to_string(),
                })),
                body: vec![Box::new(ast::ASTNode::Assignment(ast::Assignment {
                    name: Box::new(ast::ASTNode::Identifier(ast::Identifier {
                        name: "my_variable".to_string(),
                    })),
                    value: Box::new(ast::ASTNode::StringLiteral("42".to_string())),
                }))],
            },
        ))]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_import_statement() {
        let input = "import my_module\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![Box::new(
            ast::ASTNode::Import(ast::Import {
                name: "my_module".to_string(),
            }),
        )]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_bin_op() {
        let input = "a + b\n";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let result = parser.parse().unwrap();

        let expected = ast::ASTNode::Module(ast::Module::new(vec![Box::new(ast::ASTNode::BinOp(
            ast::BinOp {
                left: Box::new(ast::ASTNode::Identifier(ast::Identifier {
                    name: "a".to_string(),
                })),
                operator: TokenType::Plus,
                right: Box::new(ast::ASTNode::Identifier(ast::Identifier {
                    name: "b".to_string(),
                })),
            },
        ))]));

        assert_eq!(result, expected);
    }
}
