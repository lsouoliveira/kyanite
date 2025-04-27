use crate::parser;

pub trait Visitor {
    fn visit_program(&mut self, program: &parser::Program);
    fn visit_name(&mut self, name: &parser::Name);
    fn visit_identifier(&mut self, identifier: &parser::Identifier);
}
