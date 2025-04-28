use crate::builtins::kya_print;
use crate::lexer::Lexer;
use crate::objects::{Context, KyaError, KyaObject, KyaObjectKind, KyaRsFunction};
use crate::parser;
use crate::visitor::Visitor;

pub struct Interpreter {
    input: String,
    context: Context,
}

fn setup_builtins(context: &mut Context) {
    context.register(
        String::from("print"),
        Box::new(KyaRsFunction::new(String::from("print"), kya_print)),
    );
}

impl Interpreter {
    pub fn new(input: String) -> Self {
        let mut context = Context::new(None);

        setup_builtins(&mut context);

        Interpreter { input, context }
    }

    pub fn eval(&mut self) {
        let lexer = Lexer::new(self.input.clone());
        let mut parser = parser::Parser::new(lexer);

        match parser.parse() {
            Ok(module) => {
                module.accept(self);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}

impl Visitor for Interpreter {
    fn visit_module(&mut self, module: &parser::Module) {
        for statement in &module.statements {
            statement.accept(self);
        }
    }

    fn visit_name(&mut self, name: &parser::Name) {
        name.identifier.accept(self);
    }

    fn visit_identifier(&mut self, identifier: &parser::Identifier) {
        if let Some(object) = self.context.get(&identifier.name) {
            match object.kind() {
                KyaObjectKind::RsFunction => {
                    if let Some(function) = object.as_any().downcast_ref::<KyaRsFunction>() {
                        function.call(&self.context).unwrap();
                    }
                }
                _ => {}
            }
        } else {
            eprintln!("Undefined identifier: {}", identifier.name);
        }
    }
}
