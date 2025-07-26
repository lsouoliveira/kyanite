use crate::errors::Error;
use crate::lexer::TokenType;
use crate::visitor::{CompilerVisitor, Visitor};

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Module(Module),
    // Statements
    While(While),
    Break(),
    Block(Block),
    // Expressions
    Identifier(Identifier),
    StringLiteral(String),
    NumberLiteral(f64),
    MethodCall(MethodCall),
    Assignment(Assignment),
    MethodDef(MethodDef),
    ClassDef(ClassDef),
    Attribute(Attribute),
    Compare(Compare),
    If(If),
    Import(Import),
    BinOp(BinOp),
    UnaryOp(UnaryOp),
}

impl ASTNode {
    pub fn is_expression(&self) -> bool {
        matches!(
            self,
            ASTNode::Identifier(_)
                | ASTNode::StringLiteral(_)
                | ASTNode::NumberLiteral(_)
                | ASTNode::MethodCall(_)
                | ASTNode::Assignment(_)
                | ASTNode::Attribute(_)
                | ASTNode::Compare(_)
                | ASTNode::BinOp(_)
                | ASTNode::UnaryOp(_)
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Module {
    pub block: Box<ASTNode>,
}

impl Module {
    pub fn new(block: Box<ASTNode>) -> Self {
        Module { block }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Identifier { name }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodCall {
    pub name: Box<ASTNode>,
    pub arguments: Vec<Box<ASTNode>>,
}

impl MethodCall {
    pub fn new(name: Box<ASTNode>, arguments: Vec<Box<ASTNode>>) -> Self {
        MethodCall { name, arguments }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Assignment {
    pub name: Box<ASTNode>,
    pub value: Box<ASTNode>,
}

impl Assignment {
    pub fn new(name: Box<ASTNode>, value: Box<ASTNode>) -> Self {
        Assignment { name, value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodDef {
    pub name: String,
    pub parameters: Vec<Box<ASTNode>>,
    pub body: Box<ASTNode>,
}

impl MethodDef {
    pub fn new(name: String, parameters: Vec<Box<ASTNode>>, body: Box<ASTNode>) -> Self {
        MethodDef {
            name,
            parameters,
            body,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassDef {
    pub name: String,
    pub body: Box<ASTNode>,
}

impl ClassDef {
    pub fn new(name: String, body: Box<ASTNode>) -> Self {
        ClassDef { name, body }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
    pub name: Box<ASTNode>,
    pub value: String,
}

impl Attribute {
    pub fn new(name: Box<ASTNode>, value: String) -> Self {
        Attribute { name, value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Equal,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Compare {
    pub left: Box<ASTNode>,
    pub operator: Operator,
    pub right: Box<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct If {
    pub test: Box<ASTNode>,
    pub body: Box<ASTNode>,
}

impl If {
    pub fn new(test: Box<ASTNode>, body: Box<ASTNode>) -> Self {
        If { test, body }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Import {
    pub name: String,
}

impl Import {
    pub fn new(name: String) -> Self {
        Import { name }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct While {
    pub condition: Box<ASTNode>,
    pub body: Box<ASTNode>,
}

impl While {
    pub fn new(condition: Box<ASTNode>, body: Box<ASTNode>) -> Self {
        While { condition, body }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<Box<ASTNode>>,
}

impl Block {
    pub fn new(statements: Vec<Box<ASTNode>>) -> Self {
        Block { statements }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinOp {
    pub left: Box<ASTNode>,
    pub operator: TokenType,
    pub right: Box<ASTNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOp {
    pub operator: TokenType,
    pub operand: Box<ASTNode>,
}

impl ASTNode {
    pub fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            ASTNode::Module(module) => visitor.visit_module(&module),
            ASTNode::Identifier(identifier) => visitor.visit_identifier(&identifier),
            ASTNode::StringLiteral(string_literal) => visitor.visit_string_literal(string_literal),
            ASTNode::MethodCall(method_call) => visitor.visit_method_call(&method_call),
            ASTNode::Assignment(assignment) => visitor.visit_assignment(&assignment),
            ASTNode::NumberLiteral(number_literal) => visitor.visit_number_literal(&number_literal),
            ASTNode::MethodDef(method_def) => visitor.visit_method_def(&method_def),
            ASTNode::ClassDef(class_def) => visitor.visit_class_def(&class_def),
            ASTNode::Attribute(attribute) => visitor.visit_attribute(&attribute),
            ASTNode::Compare(compare) => visitor.visit_compare(&compare),
            ASTNode::If(if_node) => visitor.visit_if(&if_node),
            ASTNode::Import(import) => visitor.visit_import(&import),
            ASTNode::BinOp(bin_op) => visitor.visit_bin_op(&bin_op),
            ASTNode::UnaryOp(unary_op) => visitor.visit_unary_op(&unary_op),
            ASTNode::While(while_node) => visitor.visit_while(&while_node),
            ASTNode::Break() => visitor.visit_break(),
            ASTNode::Block(block) => visitor.visit_block(&block),
        }
    }

    pub fn compile(&self, compiler: &mut dyn CompilerVisitor) -> Result<(), Error> {
        match self {
            ASTNode::Module(module) => compiler.compile_module(&module),
            ASTNode::Identifier(identifier) => compiler.compile_identifier(&identifier),
            ASTNode::StringLiteral(string_literal) => {
                compiler.compile_string_literal(string_literal)
            }
            ASTNode::MethodCall(method_call) => compiler.compile_method_call(&method_call),
            ASTNode::Assignment(assignment) => compiler.compile_assignment(&assignment),
            ASTNode::NumberLiteral(number_literal) => {
                compiler.compile_number_literal(&number_literal)
            }
            ASTNode::MethodDef(method_def) => compiler.compile_method_def(&method_def),
            ASTNode::ClassDef(class_def) => compiler.compile_class_def(&class_def),
            ASTNode::Attribute(attribute) => compiler.compile_attribute(&attribute),
            ASTNode::Compare(compare) => compiler.compile_compare(&compare),
            ASTNode::If(if_node) => compiler.compile_if(&if_node),
            ASTNode::Import(import) => compiler.compile_import(&import),
            ASTNode::BinOp(bin_op) => compiler.compile_bin_op(&bin_op),
            ASTNode::UnaryOp(unary_op) => compiler.compile_unary_op(&unary_op),
            ASTNode::While(while_node) => compiler.compile_while(&while_node),
            ASTNode::Break() => compiler.compile_break(),
            ASTNode::Block(block) => compiler.compile_block(&block),
        }
    }
}
