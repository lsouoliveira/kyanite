use crate::objects::{KyaError, KyaObject};
use crate::parser;

pub trait Visitor {
    fn visit_module(&mut self, module: &parser::Module);
    fn visit_name(&mut self, name: &parser::Name);
    fn visit_identifier(&mut self, identifier: &parser::Identifier);
}

pub trait Evaluator {
    fn eval_module(&mut self, module: &parser::Module) -> Result<Box<dyn KyaObject>, KyaError>;
    fn eval_name(&mut self, name: &parser::Name) -> Result<Box<dyn KyaObject>, KyaError>;
    fn eval_identifier(
        &mut self,
        identifier: &parser::Identifier,
    ) -> Result<Box<dyn KyaObject>, KyaError>;
}
