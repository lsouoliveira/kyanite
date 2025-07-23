use crate::ast::{
    Assignment, Attribute, BinOp, Block, ClassDef, Compare, Identifier, If, Import, MethodCall,
    MethodDef, Module, UnaryOp, While,
};
use crate::errors::Error;
use crate::objects::base::KyaObjectRef;

pub trait Visitor {
    fn visit_module(&mut self, module: &Module);
    fn visit_identifier(&mut self, identifier: &Identifier);
    fn visit_method_call(&mut self, method_call: &MethodCall);
    fn visit_string_literal(&mut self, string_literal: &str);
    fn visit_assignment(&mut self, assignment: &Assignment);
    fn visit_number_literal(&mut self, number_literal: &f64);
    fn visit_method_def(&mut self, method_def: &MethodDef);
    fn visit_class_def(&mut self, class_def: &ClassDef);
    fn visit_attribute(&mut self, attribute: &Attribute);
    fn visit_compare(&mut self, compare: &Compare);
    fn visit_if(&mut self, if_node: &If);
    fn visit_import(&mut self, import: &Import);
    fn visit_bin_op(&mut self, bin_op: &BinOp);
    fn visit_unary_op(&mut self, unary_op: &UnaryOp);
    fn visit_while(&mut self, while_node: &While);
    fn visit_break(&mut self);
    fn visit_block(&mut self, block: &Block);
}

pub trait CompilerVisitor {
    fn compile_module(&mut self, module: &Module) -> Result<(), Error>;
    fn compile_identifier(&mut self, identifier: &Identifier) -> Result<(), Error>;
    fn compile_method_call(&mut self, method_call: &MethodCall) -> Result<(), Error>;
    fn compile_string_literal(&mut self, string_literal: &str) -> Result<(), Error>;
    fn compile_assignment(&mut self, assignment: &Assignment) -> Result<(), Error>;
    fn compile_number_literal(&mut self, number_literal: &f64) -> Result<(), Error>;
    fn compile_method_def(&mut self, method_def: &MethodDef) -> Result<(), Error>;
    fn compile_class_def(&mut self, class_def: &ClassDef) -> Result<(), Error>;
    fn compile_attribute(&mut self, attribute: &Attribute) -> Result<(), Error>;
    fn compile_compare(&mut self, compare: &Compare) -> Result<(), Error>;
    fn compile_if(&mut self, if_node: &If) -> Result<(), Error>;
    fn compile_import(&mut self, import: &Import) -> Result<(), Error>;
    fn compile_bin_op(&mut self, bin_op: &BinOp) -> Result<(), Error>;
    fn compile_unary_op(&mut self, unary_op: &UnaryOp) -> Result<(), Error>;
    fn compile_while(&mut self, while_node: &While) -> Result<(), Error>;
    fn compile_break(&mut self) -> Result<(), Error>;
    fn compile_block(&mut self, block: &Block) -> Result<(), Error>;
}
