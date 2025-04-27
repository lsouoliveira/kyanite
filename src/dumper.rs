use crate::parser;
use crate::visitor::Visitor;

pub struct ASTDumper {
    pub output: String,
    indent: i32,
}

impl ASTDumper {
    pub fn new() -> Self {
        ASTDumper {
            output: String::new(),
            indent: 0,
        }
    }

    fn push(&mut self, text: &str) {
        for _ in 0..self.indent {
            self.output.push_str(" ");
        }
        self.output.push_str(text);
        self.output.push_str("\n");
    }

    fn concat(&mut self, text: &str) {
        self.output.push_str(text);
    }
}

impl Visitor for ASTDumper {
    fn visit_program(&mut self, program: &parser::Program) {
        self.push("Program(");
        for statement in &program.statements {
            statement.accept(self);
        }
        self.concat(")");
    }

    fn visit_name(&mut self, name: &parser::Name) {
        self.push("Name(");
        name.identifier.accept(self);
        self.push(")");
    }

    fn visit_identifier(&mut self, identifier: &parser::Identifier) {
        self.push(&format!("Identifier: {}", identifier.name));
    }
}
