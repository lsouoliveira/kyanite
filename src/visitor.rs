use crate::ast::{
    Assignment, Attribute, ClassDef, Compare, Identifier, If, Import, MethodCall, MethodDef, Module,
};
use crate::errors::Error;
use crate::objects;
use std::rc::Rc;

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
}

pub trait Evaluator {
    fn eval_module(&mut self, module: &Module) -> Result<Rc<objects::KyaObject>, Error>;
    fn eval_identifier(&mut self, identifier: &Identifier)
        -> Result<Rc<objects::KyaObject>, Error>;
    fn eval_method_call(
        &mut self,
        method_call: &MethodCall,
    ) -> Result<Rc<objects::KyaObject>, Error>;
    fn eval_string_literal(
        &mut self,
        string_literal: &str,
    ) -> Result<Rc<objects::KyaObject>, Error>;
    fn eval_assignment(&mut self, assignment: &Assignment)
        -> Result<Rc<objects::KyaObject>, Error>;
    fn eval_number_literal(
        &mut self,
        number_literal: &f64,
    ) -> Result<Rc<objects::KyaObject>, Error>;
    fn eval_method_def(&mut self, method_def: &MethodDef) -> Result<Rc<objects::KyaObject>, Error>;
    fn eval_class_def(&mut self, class_def: &ClassDef) -> Result<Rc<objects::KyaObject>, Error>;
    fn eval_attribute(&mut self, attribute: &Attribute) -> Result<Rc<objects::KyaObject>, Error>;
    fn eval_compare(&mut self, compare: &Compare) -> Result<Rc<objects::KyaObject>, Error>;
    fn eval_if(&mut self, if_node: &If) -> Result<Rc<objects::KyaObject>, Error>;
    fn eval_import(&mut self, import: &Import) -> Result<Rc<objects::KyaObject>, Error>;
}
