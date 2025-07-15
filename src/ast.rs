use crate::errors::Error;
use crate::lexer::TokenType;
use crate::objects::KyaObject;
use crate::visitor::{Evaluator, Visitor};
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Module(Module),
    // Statements
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
}

#[derive(Debug, PartialEq, Clone)]
pub struct Module {
    pub statements: Vec<Box<ASTNode>>,
}

impl Module {
    pub fn new(statements: Vec<Box<ASTNode>>) -> Self {
        Module { statements }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub name: String,
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
    pub body: Vec<Box<ASTNode>>,
}

impl MethodDef {
    pub fn new(name: String, parameters: Vec<Box<ASTNode>>, body: Vec<Box<ASTNode>>) -> Self {
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
    pub body: Vec<Box<ASTNode>>,
    pub parameters: Vec<Box<ASTNode>>,
}

impl ClassDef {
    pub fn new(name: String, body: Vec<Box<ASTNode>>, parameters: Vec<Box<ASTNode>>) -> Self {
        ClassDef {
            name,
            body,
            parameters,
        }
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
    pub body: Vec<Box<ASTNode>>,
}

impl If {
    pub fn new(test: Box<ASTNode>, body: Vec<Box<ASTNode>>) -> Self {
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
pub struct BinOp {
    pub left: Box<ASTNode>,
    pub operator: TokenType,
    pub right: Box<ASTNode>,
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
        }
    }

    pub fn eval(&self, evaluator: &mut dyn Evaluator) -> Result<Rc<KyaObject>, Error> {
        match self {
            ASTNode::Module(module) => evaluator.eval_module(&module),
            ASTNode::Identifier(identifier) => evaluator.eval_identifier(&identifier),
            ASTNode::StringLiteral(string_literal) => evaluator.eval_string_literal(string_literal),
            ASTNode::MethodCall(method_call) => evaluator.eval_method_call(&method_call),
            ASTNode::Assignment(assignment) => evaluator.eval_assignment(&assignment),
            ASTNode::NumberLiteral(number_literal) => {
                evaluator.eval_number_literal(&number_literal)
            }
            ASTNode::MethodDef(method_def) => evaluator.eval_method_def(&method_def),
            ASTNode::ClassDef(class_def) => evaluator.eval_class_def(&class_def),
            ASTNode::Attribute(attribute) => evaluator.eval_attribute(&attribute),
            ASTNode::Compare(compare) => evaluator.eval_compare(&compare),
            ASTNode::If(if_node) => evaluator.eval_if(&if_node),
            ASTNode::Import(import) => evaluator.eval_import(&import),
            ASTNode::BinOp(bin_op) => evaluator.eval_bin_op(&bin_op),
        }
    }
}
