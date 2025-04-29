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
    fn visit_module(&mut self, module: &parser::Module) {
        self.concat("Module(");
        self.push_newline();
        for statement in &module.statements {
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

    fn visit_function_call(&mut self, function_call: &parser::FunctionCall) {
        self.push("FunctionCall(");
        function_call.func.accept(self);
        self.push("(");
        for (i, arg) in function_call.arguments.iter().enumerate() {
            arg.accept(self);
            if i < function_call.arguments.len() - 1 {
                self.concat(", ");
            }
        }
        self.concat("))");
    }

    fn visit_string_literal(&mut self, string_literal: &parser::StringLiteral) {
        self.push(&format!("StringLiteral: {}", string_literal.value));
    }
}
