use crate::lexer::Lexer;
use crate::parser;
use crate::visitor::Visitor;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn eval(&mut self, input: &str) {
        let lexer = Lexer::new(input.to_string());
        let mut parser = parser::Parser::new(lexer);

        match parser.parse() {
            Ok(module) => {
                module.accept(self);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}

impl Visitor for Interpreter {
    fn visit_module(&mut self, module: &parser::Module) {
        for statement in &module.statements {
            statement.accept(self);
        }
    }

    fn visit_name(&mut self, name: &parser::Name) {
        name.identifier.accept(self);
    }

    fn visit_identifier(&mut self, identifier: &parser::Identifier) {
        println!("Identifier: {}", identifier.name);
    }
}
