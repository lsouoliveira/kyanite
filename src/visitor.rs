use crate::parser;

pub trait Visitor {
    fn visit_module(&mut self, module: &parser::Module);
    fn visit_name(&mut self, name: &parser::Name);
    fn visit_identifier(&mut self, identifier: &parser::Identifier);
}
