use crate::bytecode::{CodeObject, ComparisonOperator, Opcode};
use crate::errors::Error;
use crate::objects::code_object::code_object_new;
use crate::objects::function_object::function_new;
use crate::objects::number_object::number_new;
use crate::objects::string_object::string_new;
use crate::{ast, visitor::CompilerVisitor};

use std::sync::Arc;

pub struct Compiler {
    ast: Arc<ast::ASTNode>,
    code: CodeObject,
}

impl Compiler {
    pub fn new(ast: Arc<ast::ASTNode>) -> Self {
        Compiler {
            ast,
            code: CodeObject::new(),
        }
    }

    pub fn compile(&mut self) -> Result<(), Error> {
        self.ast.clone().compile(self)
    }

    pub fn get_output(&self) -> CodeObject {
        self.code.clone()
    }

    fn store_variable(&mut self, name: String) {
        let index = self.code.add_name(name);

        self.code.add_instruction(Opcode::StoreName as u8);
        self.code.add_instruction(index);
    }

    fn load_variable(&mut self, name: String) {
        let index = self.code.add_name(name);

        self.code.add_instruction(Opcode::LoadName as u8);
        self.code.add_instruction(index);
    }

    fn load_attr(&mut self, value: &str) {
        self.code.add_instruction(Opcode::LoadAttr as u8);
        let index = self.code.add_name(value.to_string());
        self.code.add_instruction(index);
    }
}

impl CompilerVisitor for Compiler {
    fn compile_module(&mut self, module: &ast::Module) -> Result<(), Error> {
        module.block.compile(self)?;

        Ok(())
    }

    fn compile_identifier(&mut self, identifier: &ast::Identifier) -> Result<(), Error> {
        self.load_variable(identifier.name.clone());

        Ok(())
    }

    fn compile_method_call(&mut self, method_call: &ast::MethodCall) -> Result<(), Error> {
        method_call.name.compile(self)?;

        for arg in &method_call.arguments {
            arg.compile(self)?;
        }

        let arg_count = method_call.arguments.len() as u8;

        self.code.add_instruction(Opcode::Call as u8);
        self.code.add_instruction(arg_count);

        Ok(())
    }

    fn compile_string_literal(&mut self, string_literal: &str) -> Result<(), Error> {
        let object = string_new(string_literal);

        let index = self.code.add_const(object);

        self.code.add_instruction(Opcode::LoadConst as u8);
        self.code.add_instruction(index);

        Ok(())
    }

    fn compile_assignment(&mut self, assignment: &ast::Assignment) -> Result<(), Error> {
        assignment.value.compile(self)?;

        if let ast::ASTNode::Identifier(identifier) = &*assignment.name {
            self.store_variable(identifier.name.clone());
            self.load_variable(identifier.name.clone());
        } else {
            return Err(Error::CompilationError(
                "Assignment name must be an identifier".to_string(),
            ));
        }

        Ok(())
    }

    fn compile_number_literal(&mut self, number_literal: &f64) -> Result<(), Error> {
        let object = number_new(*number_literal);

        let index = self.code.add_const(object);

        self.code.add_instruction(Opcode::LoadConst as u8);
        self.code.add_instruction(index);

        Ok(())
    }

    fn compile_method_def(&mut self, method_def: &ast::MethodDef) -> Result<(), Error> {
        let mut compiler = Compiler::new(Arc::new(*method_def.body.clone()));
        let _ = compiler.compile()?;
        let mut code = compiler.get_output();

        for param in &method_def.parameters {
            if let ast::ASTNode::Identifier(identifier) = &**param {
                code.args.push(identifier.name.clone());
            } else {
                return Err(Error::CompilationError(
                    "Method parameters must be identifiers".to_string(),
                ));
            }
        }

        code.name = method_def.name.clone();

        let code_object = code_object_new(Arc::new(code));

        let index = self.code.add_const(code_object);
        self.code.add_instruction(Opcode::LoadConst as u8);
        self.code.add_instruction(index);

        self.code.add_instruction(Opcode::MakeFunction as u8);

        Ok(())
    }

    fn compile_class_def(&mut self, class_def: &ast::ClassDef) -> Result<(), Error> {
        Ok(())
    }

    fn compile_attribute(&mut self, attribute: &ast::Attribute) -> Result<(), Error> {
        attribute.name.compile(self)?;
        self.load_attr(&attribute.value);

        Ok(())
    }

    fn compile_compare(&mut self, compare: &ast::Compare) -> Result<(), Error> {
        compare.left.compile(self)?;
        compare.right.compile(self)?;

        self.code.add_instruction(Opcode::Compare as u8);
        self.code.add_instruction(ComparisonOperator::Equal as u8);

        Ok(())
    }

    fn compile_if(&mut self, if_node: &ast::If) -> Result<(), Error> {
        Ok(())
    }

    fn compile_import(&mut self, import: &ast::Import) -> Result<(), Error> {
        Ok(())
    }

    fn compile_bin_op(&mut self, bin_op: &ast::BinOp) -> Result<(), Error> {
        Ok(())
    }

    fn compile_unary_op(&mut self, unary_op: &ast::UnaryOp) -> Result<(), Error> {
        Ok(())
    }

    fn compile_while(&mut self, while_node: &ast::While) -> Result<(), Error> {
        Ok(())
    }

    fn compile_break(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn compile_block(&mut self, block: &ast::Block) -> Result<(), Error> {
        for statement in &block.statements {
            statement.compile(self)?;

            if statement.is_expression() {
                self.code.add_instruction(Opcode::PopTop as u8);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, Module};

    #[test]
    fn test_compile_string() {
        let ast = Arc::new(ASTNode::Module(Module {
            statements: vec![Box::new(ASTNode::StringLiteral(
                "Hello, World!".to_string(),
            ))],
        }));

        let mut compiler = Compiler::new(ast);
        compiler.compile().unwrap();

        let expected_code = vec![
            Opcode::LoadConst as u8,
            0, // index of "Hello, World!" in consts
        ];

        assert_eq!(compiler.get_output().code, expected_code);
    }

    #[test]
    fn test_compile_assignment() {
        let ast = Arc::new(ASTNode::Module(Module {
            statements: vec![Box::new(ASTNode::Assignment(ast::Assignment {
                name: Box::new(ASTNode::Identifier(ast::Identifier {
                    name: "x".to_string(),
                })),
                value: Box::new(ASTNode::NumberLiteral(42.0)),
            }))],
        }));

        let mut compiler = Compiler::new(ast);
        compiler.compile().unwrap();

        let expected_code = vec![
            Opcode::LoadConst as u8,
            0, // index of 42.0 in consts
            Opcode::StoreName as u8,
            0, // index of "x" in names
        ];

        assert_eq!(compiler.get_output().code, expected_code);
    }
}
