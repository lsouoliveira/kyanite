use crate::bytecode::{CodeObject, ComparisonOperator, Opcode, Operator};
use crate::errors::Error;
use crate::objects::code_object::code_object_new;
use crate::objects::function_object::function_new;
use crate::objects::number_object::number_new;
use crate::objects::string_object::string_new;
use crate::{ast, visitor::CompilerVisitor};

use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
enum ScopeType {
    Function,
    While,
}

pub struct Scope {
    scope_type: ScopeType,
    jumps: Vec<usize>,
}

pub struct Compiler {
    ast: Arc<ast::ASTNode>,
    code: CodeObject,
    scopes: Vec<Scope>,
}

impl Compiler {
    pub fn new(ast: Arc<ast::ASTNode>) -> Self {
        Compiler {
            ast,
            code: CodeObject::new(),
            scopes: vec![],
        }
    }

    pub fn compile(&mut self) -> Result<(), Error> {
        self.ast.clone().compile(self)?;

        Ok(())
    }

    pub fn get_output(&self) -> CodeObject {
        self.code.clone()
    }

    fn enter_scope(&mut self, scope_type: ScopeType) {
        self.scopes.push(Scope {
            scope_type,
            jumps: vec![],
        });
    }

    fn exit_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            for jump in scope.jumps {
                self.code
                    .set_instruction_at(jump, self.code.instructions_count() as u8);
            }
        }
    }

    fn current_scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    fn push_jump(&mut self, jump: usize) {
        self.current_scope().jumps.push(jump);
    }

    fn backpatch(&mut self, target: usize) {
        for jump in self.current_scope().jumps.clone() {
            self.code.set_instruction_at(jump, target as u8);
        }
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

    fn store_attr(&mut self, value: &str) {
        self.code.add_instruction(Opcode::StoreAttr as u8);
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
        } else if let ast::ASTNode::Attribute(attribute) = &*assignment.name {
            attribute.name.compile(self)?;
            self.store_attr(&attribute.value);
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

        compiler.enter_scope(ScopeType::Function);

        let _ = compiler.compile()?;

        compiler.exit_scope();

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
        let mut compiler = Compiler::new(Arc::new(*class_def.body.clone()));
        let _ = compiler.compile()?;
        let mut code = compiler.get_output();

        code.name = class_def.name.clone();

        let code_object = code_object_new(Arc::new(code));

        let index = self.code.add_const(code_object);
        self.code.add_instruction(Opcode::LoadConst as u8);
        self.code.add_instruction(index);

        self.code.add_instruction(Opcode::MakeClass as u8);

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

        let operator =
            if let Some(op) = ComparisonOperator::from_ast_operator(compare.operator.clone()) {
                op
            } else {
                return Err(Error::CompilationError(
                    "Comparison operator is missing".to_string(),
                ));
            };

        self.code.add_instruction(Opcode::Compare as u8);
        self.code.add_instruction(operator as u8);

        Ok(())
    }

    fn compile_if(&mut self, if_node: &ast::If) -> Result<(), Error> {
        if_node.test.compile(self)?;

        self.code.add_instruction(Opcode::PopAndJumpIfFalse as u8);
        self.code.add_instruction(0);

        let jump_index = self.code.instructions_count() as u8 - 1;

        if_node.body.compile(self)?;

        self.code
            .set_instruction_at(jump_index as usize, self.code.instructions_count() as u8);

        Ok(())
    }

    fn compile_import(&mut self, import: &ast::Import) -> Result<(), Error> {
        Ok(())
    }

    fn compile_bin_op(&mut self, bin_op: &ast::BinOp) -> Result<(), Error> {
        bin_op.left.compile(self)?;
        bin_op.right.compile(self)?;
        let operator = if let Some(op) = Operator::from_ast_operator(bin_op.operator.clone()) {
            op
        } else {
            return Err(Error::CompilationError(
                "Binary operator is missing".to_string(),
            ));
        };
        self.code.add_instruction(Opcode::BinaryOp as u8);
        self.code.add_instruction(operator as u8);

        Ok(())
    }

    fn compile_unary_op(&mut self, unary_op: &ast::UnaryOp) -> Result<(), Error> {
        Ok(())
    }

    fn compile_while(&mut self, while_node: &ast::While) -> Result<(), Error> {
        self.enter_scope(ScopeType::While);

        let condition_target = self.code.instructions_count() as u8;

        while_node.condition.compile(self)?;

        self.code.add_instruction(Opcode::PopAndJumpIfFalse as u8);

        let jump_target = self.code.instructions_count() as u8;

        self.code.add_instruction(0);
        self.push_jump(jump_target as usize);

        while_node.body.compile(self)?;

        let end_target = self.code.instructions_count() as u8;
        let jump_offset = end_target - condition_target + 2;

        self.code.add_instruction(Opcode::JumpBack as u8);
        self.code.add_instruction(jump_offset);

        self.backpatch(self.code.instructions_count() - 1);

        self.exit_scope();

        Ok(())
    }

    fn compile_break(&mut self) -> Result<(), Error> {
        if self.scopes.is_empty() || self.current_scope().scope_type != ScopeType::While {
            return Err(Error::SyntaxError(
                "Break statement outside of loop".to_string(),
            ));
        }

        self.code.add_instruction(Opcode::Jump as u8);
        self.code.add_instruction(0);
        self.push_jump(self.code.instructions_count() - 1);

        Ok(())
    }

    fn compile_block(&mut self, block: &ast::Block) -> Result<(), Error> {
        for statement in &block.statements {
            if let ast::ASTNode::Break() = &**statement {
                statement.compile(self)?;

                return Ok(());
            }

            statement.compile(self)?;

            if statement.is_expression() {
                self.code.add_instruction(Opcode::PopTop as u8);
            }
        }

        Ok(())
    }

    fn compile_return(&mut self, return_node: &ast::Return) -> Result<(), Error> {
        if self.scopes.is_empty() || self.current_scope().scope_type != ScopeType::Function {
            return Err(Error::SyntaxError(
                "Return statement outside of function".to_string(),
            ));
        }

        if let Some(value) = &return_node.value {
            value.compile(self)?;
        } else {
            self.load_variable("None".to_string());
        }

        self.code.add_instruction(Opcode::Return as u8);

        Ok(())
    }

    fn compile_raise(&mut self, raise: &ast::Raise) -> Result<(), Error> {
        if let Some(message) = &raise.message {
            message.compile(self)?;
        } else {
            self.load_variable("None".to_string());
        }

        self.code.add_instruction(Opcode::Raise as u8);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{ASTNode, Module},
        objects::base::KyaObject,
    };

    #[test]
    fn test_compile_while() {
        let condition = ASTNode::Compare(ast::Compare {
            left: Box::new(ASTNode::Identifier(ast::Identifier::new("x".to_string()))),
            operator: ast::Operator::Equal,
            right: Box::new(ASTNode::NumberLiteral(0.0)),
        });

        let body = ASTNode::Block(ast::Block::new(vec![Box::new(ASTNode::Identifier(
            ast::Identifier::new("x".to_string()),
        ))]));

        let while_node = ASTNode::While(ast::While {
            condition: Box::new(condition),
            body: Box::new(body),
        });

        let mut compiler = Compiler::new(Arc::new(while_node));
        let _ = compiler.compile();

        let code_object = compiler.get_output();

        let expected_output = vec![
            Opcode::LoadName as u8,  // Load variable 'x'
            0,                       // Index for 'x'
            Opcode::LoadConst as u8, // Load constant 0.0
            0,                       // Index for constant 0.0
            Opcode::Compare as u8,   // Compare x == 0.0
            ComparisonOperator::Equal as u8,
            Opcode::PopAndJumpIfFalse as u8, // Jump if condition is false
            13,                              // Jump target
            Opcode::LoadName as u8,          // Load variable 'x' again in the body
            0,                               // Index for 'x'
            Opcode::PopTop as u8,            // Pop the result of the body
            Opcode::JumpBack as u8,          // Jump back to the condition check
            13,                              // Offset to jump back to the condition check
        ];

        assert_eq!(expected_output, code_object.code);
    }

    #[test]
    fn test_compile_with_break() {
        let condition = ASTNode::Compare(ast::Compare {
            left: Box::new(ASTNode::Identifier(ast::Identifier::new("x".to_string()))),
            operator: ast::Operator::Equal,
            right: Box::new(ASTNode::NumberLiteral(0.0)),
        });

        let body = ASTNode::Block(ast::Block::new(vec![
            Box::new(ASTNode::Identifier(ast::Identifier::new("x".to_string()))),
            Box::new(ASTNode::Break()),
        ]));

        let while_node = ASTNode::While(ast::While {
            condition: Box::new(condition),
            body: Box::new(body),
        });

        let mut compiler = Compiler::new(Arc::new(while_node));
        let _ = compiler.compile();

        let code_object = compiler.get_output();

        let expected_output = vec![
            Opcode::LoadName as u8,  // Load variable 'x'
            0,                       // Index for 'x'
            Opcode::LoadConst as u8, // Load constant 0.0
            0,                       // Index for constant 0.0
            Opcode::Compare as u8,   // Compare x == 0.0
            ComparisonOperator::Equal as u8,
            Opcode::PopAndJumpIfFalse as u8, // Jump if condition is false
            15,                              // Jump target
            Opcode::LoadName as u8,          // Load variable 'x' again in the body
            0,                               // Index for 'x'
            Opcode::PopTop as u8,            // Pop the result of the body
            Opcode::Jump as u8,              // Jump to the end of the loop
            15,                              // Offset to jump to the end of the loop
            Opcode::JumpBack as u8,          // Jump back to the condition check
            15,
        ];

        assert_eq!(expected_output, code_object.code);
    }

    #[test]
    fn test_if() {
        let condition = ASTNode::Compare(ast::Compare {
            left: Box::new(ASTNode::Identifier(ast::Identifier::new("x".to_string()))),
            operator: ast::Operator::Equal,
            right: Box::new(ASTNode::NumberLiteral(0.0)),
        });

        let body = ASTNode::Block(ast::Block::new(vec![Box::new(ASTNode::Identifier(
            ast::Identifier::new("x".to_string()),
        ))]));

        let if_node = ASTNode::If(ast::If {
            test: Box::new(condition),
            body: Box::new(body),
        });

        let mut compiler = Compiler::new(Arc::new(if_node));
        let _ = compiler.compile();

        let code_object = compiler.get_output();

        let expected_output = vec![
            Opcode::LoadName as u8,  // Load variable 'x'
            0,                       // Index for 'x'
            Opcode::LoadConst as u8, // Load constant 0.0
            0,                       // Index for constant 0.0
            Opcode::Compare as u8,   // Compare x == 0.0
            ComparisonOperator::Equal as u8,
            Opcode::PopAndJumpIfFalse as u8, // Jump if condition is false
            11,                              // Jump target
            Opcode::LoadName as u8,          // Load variable 'x' in the body
            0,                               // Index for 'x'
            Opcode::PopTop as u8,            // Pop the result of the body
        ];

        assert_eq!(expected_output, code_object.code);
    }

    #[test]
    fn test_compile_class() {
        let class_def = ASTNode::ClassDef(ast::ClassDef {
            name: "MyClass".to_string(),
            body: Box::new(ASTNode::Block(ast::Block::new(vec![]))),
        });

        let mut compiler = Compiler::new(Arc::new(class_def));
        let _ = compiler.compile();

        let code_object = compiler.get_output();

        let expected_output = vec![
            Opcode::LoadConst as u8, // Load class definition
            0,                       // Index for class definition
            Opcode::MakeClass as u8, // Create class object
        ];

        assert_eq!(expected_output, code_object.code);
    }

    #[test]
    fn test_compile_return() {
        let return_node = ASTNode::MethodDef(ast::MethodDef {
            name: "my_method".to_string(),
            parameters: vec![Box::new(ASTNode::Identifier(ast::Identifier::new(
                "x".to_string(),
            )))],
            body: Box::new(ASTNode::Block(ast::Block::new(vec![Box::new(
                ASTNode::Return(ast::Return {
                    value: Some(Box::new(ASTNode::Identifier(ast::Identifier::new(
                        "x".to_string(),
                    )))),
                }),
            )]))),
        });

        let mut compiler = Compiler::new(Arc::new(return_node));
        let _ = compiler.compile();

        let code_object = compiler.get_output();
        let function_code_object = code_object.consts[0].lock().unwrap();
        let function_code_object = match &*function_code_object {
            KyaObject::CodeObject(code_object) => code_object,
            _ => panic!("Expected CodeObject"),
        };

        let expected_output = vec![
            Opcode::LoadName as u8, // Load variable 'x'
            0,                      // Index for 'x'
            Opcode::Return as u8,   // Return from method
        ];

        assert_eq!(expected_output, function_code_object.code.code);
    }

    #[test]
    fn test_compile_bin_op() {
        let bin_op = ASTNode::BinOp(ast::BinOp {
            left: Box::new(ASTNode::NumberLiteral(5.0)),
            operator: ast::Operator::Plus,
            right: Box::new(ASTNode::NumberLiteral(3.0)),
        });

        let mut compiler = Compiler::new(Arc::new(bin_op));
        let _ = compiler.compile();

        let code_object = compiler.get_output();

        let expected_output = vec![
            Opcode::LoadConst as u8, // Load constant 5.0
            0,                       // Index for constant 5.0
            Opcode::LoadConst as u8, // Load constant 3.0
            1,                       // Index for constant 3.0
            Opcode::BinaryOp as u8,  // Perform addition
            Operator::Plus as u8,
        ];

        assert_eq!(expected_output, code_object.code);
    }
}
