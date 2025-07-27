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
        module.block.accept(self);
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

    fn visit_assignment(&mut self, assignment: &ast::Assignment) {
        self.push("Assignment(");
        self.concat("name: ");
        assignment.name.accept(self);
        assignment.value.accept(self);
        self.push(")");
    }

    fn visit_number_literal(&mut self, number_literal: &f64) {
        self.push(&format!("NumberLiteral({})", number_literal));
    }

    fn visit_method_def(&mut self, method_def: &ast::MethodDef) {
        self.push(&format!("MethodDef({})", method_def.name));
        self.concat("body: [");
        method_def.body.accept(self);
    }

    fn visit_class_def(&mut self, class_def: &ast::ClassDef) {
        self.push(&format!("ClassDef({})", class_def.name));
        self.concat("body: ");
        class_def.body.accept(self);
    }

    fn visit_attribute(&mut self, attribute: &ast::Attribute) {
        self.push("Attribute(");
        self.concat("name: ");
        attribute.name.accept(self);
        self.concat("value: ");
        self.push(&format!("\"{}\"", attribute.value));
        self.push(")");
    }

    fn visit_compare(&mut self, compare: &ast::Compare) {
        self.push("Compare(");
        self.concat("left: ");
        compare.left.accept(self);
        self.concat("operator: ");
        self.push(&format!("{:?}", compare.operator));
        self.concat("right: ");
        compare.right.accept(self);
        self.push(")");
    }

    fn visit_if(&mut self, if_statement: &ast::If) {
        self.push("If(");
        self.concat("test: ");
        if_statement.test.accept(self);
        self.concat("body: ");
        if_statement.body.accept(self);
        self.push(")");
    }

    fn visit_import(&mut self, import: &ast::Import) {
        self.push("Import(");
        self.concat("name: ");
        self.push(&format!("\"{}\"", import.name));
        self.push(")");
    }

    fn visit_bin_op(&mut self, bin_op: &ast::BinOp) {
        self.push("BinOp(");
        self.concat("left: ");
        bin_op.left.accept(self);
        self.concat("operator: ");
        self.push(&format!("{:?}", bin_op.operator));
        self.concat("right: ");
        bin_op.right.accept(self);
        self.push(")");
    }

    fn visit_unary_op(&mut self, unary_op: &ast::UnaryOp) {
        self.push("UnaryOp(");
        self.concat("operator: ");
        self.push(&format!("{:?}", unary_op.operator));
        self.concat("operand: ");
        unary_op.operand.accept(self);
        self.push(")");
    }

    fn visit_while(&mut self, while_node: &ast::While) {
        self.push("While(");
        self.concat("condition: ");
        while_node.condition.accept(self);
        self.concat("body: ");
        while_node.body.accept(self);
        self.push(")");
    }

    fn visit_break(&mut self) {
        self.push("Break");
    }

    fn visit_block(&mut self, block: &ast::Block) {
        self.push("Block(");
        for statement in &block.statements {
            statement.accept(self);
        }
        self.push(")");
    }

    fn visit_return(&mut self, return_node: &ast::Return) {
        self.push("Return(");
        if let Some(value) = &return_node.value {
            value.accept(self);
        } else {
            self.push("None");
        }
        self.push(")");
    }
}
