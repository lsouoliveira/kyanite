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

pub struct Name {
    pub identifier: Box<dyn ASTNode>,
}

pub struct Identifier {
    pub name: String,
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
        let expr = self.parse_expression()?;

        self.skip_newlines();

        Ok(expr)
    }

    fn parse_expression(&mut self) -> Result<Box<dyn ASTNode>, ParserError> {
        let token = self.current_token.clone();

        self.expect(TokenType::Identifier).unwrap();

        return Ok(Box::new(Identifier {
            name: token.unwrap().value,
        }));
    }

    fn expect(&mut self, token_type: TokenType) -> Result<(), ParserError> {
        if let Some(token) = &self.current_token {
            if token.kind == token_type {
                self.current_token = self.lexer.next_token().unwrap();

                return Ok(());
            }
        }

        Err(ParserError::new("Unexpected symbol"))
    }

    fn accept(&mut self, token_type: TokenType) -> Result<(), ParserError> {
        if let Some(token) = &self.current_token {
            if token.kind == token_type {
                self.current_token = self.lexer.next_token().unwrap();

                return Ok(());
            }
        }

        Err(ParserError::new("Unexpected token"))
    }

    fn next_token(&mut self) -> Result<(), ParserError> {
        self.current_token = self.lexer.next_token().unwrap();
        Ok(())
    }

    fn skip_newlines(&mut self) {
        while self.accept(TokenType::Newline).is_ok() {
            self.next_token().unwrap();
        }
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

        let identifier_node = module.statements[0]
            .as_any()
            .downcast_ref::<Identifier>()
            .unwrap();
        assert_eq!(identifier_node.name, "my_function");
    }
}
