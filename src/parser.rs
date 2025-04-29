use crate::lexer::Lexer;
use crate::lexer::{Token, TokenType};
use crate::objects::{KyaError, KyaObject};
use crate::visitor::{Evaluator, Visitor};
use std::any::Any;

pub trait ASTNode {
    fn accept(&self, visitor: &mut dyn Visitor);
    fn eval(&self, evaluator: &mut dyn Evaluator) -> Result<Box<dyn KyaObject>, KyaError>;
    fn as_any(&self) -> &dyn Any;
}

pub struct Module {
    pub statements: Vec<Box<dyn ASTNode>>,
}

pub struct Body {
    pub statements: Vec<Box<dyn ASTNode>>,
}

pub struct FunctionCall {
    pub func: Box<dyn ASTNode>,
    pub arguments: Vec<Box<dyn ASTNode>>,
}

pub struct Name {
    pub identifier: Box<dyn ASTNode>,
}

pub struct Identifier {
    pub name: String,
}

pub struct StringLiteral {
    pub value: String,
}

impl ASTNode for Module {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_module(self);
    }

    fn eval(&self, evaluator: &mut dyn Evaluator) -> Result<Box<dyn KyaObject>, KyaError> {
        evaluator.eval_module(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ASTNode for FunctionCall {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_function_call(self);
    }

    fn eval(&self, evaluator: &mut dyn Evaluator) -> Result<Box<dyn KyaObject>, KyaError> {
        evaluator.eval_function_call(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ASTNode for Name {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_name(self);
    }

    fn eval(&self, evaluator: &mut dyn Evaluator) -> Result<Box<dyn KyaObject>, KyaError> {
        evaluator.eval_name(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ASTNode for Identifier {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_identifier(self);
    }

    fn eval(&self, evaluator: &mut dyn Evaluator) -> Result<Box<dyn KyaObject>, KyaError> {
        evaluator.eval_identifier(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ASTNode for StringLiteral {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_string_literal(self);
    }

    fn eval(&self, evaluator: &mut dyn Evaluator) -> Result<Box<dyn KyaObject>, KyaError> {
        evaluator.eval_string_literal(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct ParserError {
    message: String,
}

impl ParserError {
    fn new(message: &str) -> Self {
        ParserError {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParserError: {}", self.message)
    }
}

impl std::error::Error for ParserError {}

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

    pub fn parse(&mut self) -> Result<Box<dyn ASTNode>, ParserError> {
        self.next_token()?;
        let mut module = Module { statements: vec![] };

        if self.current_token.is_none() {
            return Ok(Box::new(module));
        }

        while self.current_token.is_some() {
            match self.parse_statement() {
                Ok(statement) => module.statements.push(statement),
                Err(e) => return Err(e),
            }
        }

        Ok(Box::new(module))
    }

    fn parse_statement(&mut self) -> Result<Box<dyn ASTNode>, ParserError> {
        self.skip_newlines();

        let expr = self.parse_expression()?;

        self.skip_newlines();

        Ok(expr)
    }

    fn parse_expression(&mut self) -> Result<Box<dyn ASTNode>, ParserError> {
        if let Some(identifier) = self.accept(TokenType::Identifier).ok() {
            let name = Name {
                identifier: Box::new(Identifier {
                    name: identifier.unwrap().value,
                }),
            };

            if self.accept(TokenType::LeftParen).is_ok() {
                let mut arguments = vec![];

                if let Some(arg) = self.parse_expression().ok() {
                    arguments.push(arg);
                }

                self.expect(TokenType::RightParen)?;

                return Ok(Box::new(FunctionCall {
                    func: Box::new(name),
                    arguments,
                }));
            }

            return Ok(Box::new(name));
        } else if let Some(name) = self.accept(TokenType::StringLiteral).ok() {
            let string_literal = StringLiteral {
                value: name.unwrap().value,
            };

            return Ok(Box::new(string_literal));
        }

        Err(ParserError::new("Expected expression"))
    }

    fn expect(&mut self, token_type: TokenType) -> Result<Option<Token>, ParserError> {
        if let Some(token) = &self.current_token {
            if token.kind == token_type {
                let current_token = self.current_token.clone();
                self.next_token()?;

                return Ok(current_token);
            }
        }

        Err(ParserError::new("Unexpected symbol"))
    }

    fn accept(&mut self, token_type: TokenType) -> Result<Option<Token>, ParserError> {
        if let Some(token) = &self.current_token {
            if token.kind == token_type {
                let current_token = self.current_token.clone();
                self.next_token()?;

                return Ok(current_token);
            }
        }

        Err(ParserError::new("Unexpected token"))
    }

    fn next_token(&mut self) -> Result<(), ParserError> {
        self.current_token = self.lexer.next_token().unwrap();
        Ok(())
    }

    fn skip_newlines(&mut self) {
        while self.accept(TokenType::Newline).is_ok() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_program() {
        let input = "";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let result = parser.parse();

        assert!(result.is_ok());

        let module_node = result.unwrap();
        let module = module_node.as_any().downcast_ref::<Module>().unwrap();

        assert_eq!(module.statements.len(), 0);
    }

    #[test]
    fn test_parse_identifier() {
        let input = "my_function";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let result = parser.parse();

        assert!(result.is_ok());

        let module_node = result.unwrap();
        let module = module_node.as_any().downcast_ref::<Module>().unwrap();

        assert_eq!(module.statements.len(), 1);

        let name_node = module.statements[0]
            .as_any()
            .downcast_ref::<Name>()
            .unwrap();
        let identifier_node = name_node
            .identifier
            .as_any()
            .downcast_ref::<Identifier>()
            .unwrap();
        assert_eq!(identifier_node.name, "my_function");
    }

    #[test]
    fn test_parse_function_call() {
        let input = "my_function()";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let result = parser.parse();

        assert!(result.is_ok());

        let module_node = result.unwrap();
        let module = module_node.as_any().downcast_ref::<Module>().unwrap();

        assert_eq!(module.statements.len(), 1);

        let function_call_node = module.statements[0]
            .as_any()
            .downcast_ref::<FunctionCall>()
            .unwrap();
        assert_eq!(function_call_node.arguments.len(), 0);
    }

    #[test]
    fn test_parse_string_literal() {
        let input = "\"Hello, World!\"";
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);
        let result = parser.parse();

        assert!(result.is_ok());

        let module_node = result.unwrap();
        let module = module_node.as_any().downcast_ref::<Module>().unwrap();

        assert_eq!(module.statements.len(), 1);

        let string_literal_node = module.statements[0]
            .as_any()
            .downcast_ref::<StringLiteral>()
            .unwrap();
        assert_eq!(string_literal_node.value, "Hello, World!");
    }
}
