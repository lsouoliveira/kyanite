use crate::lexer::Lexer;
use crate::Visitor;

pub trait ASTNode {
    fn accept(&self, visitor: &mut dyn Visitor);
}

pub struct Program {
    pub statements: Vec<Box<dyn ASTNode>>,
}

pub struct Name {
    pub identifier: Box<dyn ASTNode>,
}

pub struct Identifier {
    pub name: String,
}

impl ASTNode for Program {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_program(self);
    }
}
impl ASTNode for Name {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_name(self);
    }
}
impl ASTNode for Identifier {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_identifier(self);
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
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser { lexer }
    }

    pub fn parse(&self) -> Result<Box<dyn ASTNode>, ParserError> {
        Ok(Box::new(Program { statements: vec![] }))
    }
}
