use crate::ast::{
    Assignment, Attribute, BinOp, ClassDef, Compare, Identifier, If, Import, MethodCall, MethodDef,
    Module, UnaryOp, While,
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
}

pub trait Evaluator {
    fn eval_module(&mut self, module: &Module) -> Result<KyaObjectRef, Error>;
    fn eval_identifier(&mut self, identifier: &Identifier) -> Result<KyaObjectRef, Error>;
    fn eval_method_call(&mut self, method_call: &MethodCall) -> Result<KyaObjectRef, Error>;
    fn eval_string_literal(&mut self, string_literal: &str) -> Result<KyaObjectRef, Error>;
    fn eval_assignment(&mut self, assignment: &Assignment) -> Result<KyaObjectRef, Error>;
    fn eval_number_literal(&mut self, number_literal: &f64) -> Result<KyaObjectRef, Error>;
    fn eval_method_def(&mut self, method_def: &MethodDef) -> Result<KyaObjectRef, Error>;
    fn eval_class_def(&mut self, class_def: &ClassDef) -> Result<KyaObjectRef, Error>;
    fn eval_attribute(&mut self, attribute: &Attribute) -> Result<KyaObjectRef, Error>;
    fn eval_compare(&mut self, compare: &Compare) -> Result<KyaObjectRef, Error>;
    fn eval_if(&mut self, if_node: &If) -> Result<KyaObjectRef, Error>;
    fn eval_import(&mut self, import: &Import) -> Result<KyaObjectRef, Error>;
    fn eval_bin_op(&mut self, bin_op: &BinOp) -> Result<KyaObjectRef, Error>;
    fn eval_unary_op(&mut self, unary_op: &UnaryOp) -> Result<KyaObjectRef, Error>;
    fn eval_while(&mut self, while_node: &While) -> Result<KyaObjectRef, Error>;
    fn eval_break(&mut self) -> Result<KyaObjectRef, Error>;
}
