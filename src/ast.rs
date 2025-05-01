use crate::errors::Error;
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
}

impl ClassDef {
    pub fn new(name: String, body: Vec<Box<ASTNode>>) -> Self {
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
        }
    }
}
