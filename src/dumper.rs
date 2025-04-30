use crate::ast;
use crate::visitor::Visitor;

pub struct ASTDumper {
    pub output: String,
    indent: i32,
}

impl ASTDumper {
    pub fn new() -> Self {
        ASTDumper {
            output: String::new(),
            indent: 4,
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

    fn push_newline(&mut self) {
        self.output.push_str("\n");
    }
}

impl Visitor for ASTDumper {
    fn visit_module(&mut self, module: &ast::Module) {
        self.concat("Module(");
        self.push_newline();
        for statement in &module.statements {
            statement.accept(self);
        }
        self.push(")");
    }

    fn visit_identifier(&mut self, identifier: &ast::Identifier) {
        self.push(&format!("Identifier({})", identifier.name));
    }

    fn visit_method_call(&mut self, method_call: &ast::MethodCall) {
        self.push("MethodCall(");
        self.concat("name: ");
        method_call.name.accept(self);
        self.concat("arguments: [");
        for arg in &method_call.arguments {
            arg.accept(self);
        }
        self.push("]");
        self.push(")");
    }

    fn visit_string_literal(&mut self, string_literal: &str) {
        self.push(&format!("StringLiteral({})", string_literal));
    }
}
