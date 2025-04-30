use crate::ast::{ASTNode, Identifier, MethodCall, Module};
use crate::errors::Error;
use crate::objects;
use std::rc::Rc;

pub trait Visitor {
    fn visit_module(&mut self, module: &Module);
    fn visit_identifier(&mut self, identifier: &Identifier);
    fn visit_method_call(&mut self, method_call: &MethodCall);
    fn visit_string_literal(&mut self, string_literal: &str);
}

pub trait Evaluator {
    fn eval_module(&mut self, module: &Module) -> Result<Rc<objects::KyaObject>, Error>;
    fn eval_identifier(&mut self, identifier: &Identifier) -> Result<Rc<objects::KyaObject>, Error>;
    fn eval_method_call(
        &mut self,
        method_call: &MethodCall,
    ) -> Result<Rc<objects::KyaObject>, Error>;
    fn eval_string_literal(&mut self, string_literal: &str) -> Result<Rc<objects::KyaObject>, Error>;
}
