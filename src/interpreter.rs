use crate::builtins::kya_print;
use crate::lexer::Lexer;
use crate::objects::{
    Context, KyaError, KyaNone, KyaObject, KyaObjectKind, KyaRsFunction, KyaString,
};
use crate::parser;
use crate::visitor::Evaluator;

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

    pub fn evaluate(&mut self) -> Result<(), KyaError> {
        let lexer = Lexer::new(self.input.clone());
        let mut parser = parser::Parser::new(lexer);

        match parser.parse() {
            Ok(module) => {
                module.eval(self)?;
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }

        Ok(())
    }
}

impl Evaluator for Interpreter {
    fn eval_module(&mut self, module: &parser::Module) -> Result<Box<dyn KyaObject>, KyaError> {
        for statement in &module.statements {
            statement.eval(self)?;
        }

        Ok(Box::new(KyaNone {}))
    }

    fn eval_name(&mut self, name: &parser::Name) -> Result<Box<dyn KyaObject>, KyaError> {
        name.identifier.eval(self)
    }

    fn eval_identifier(
        &mut self,
        identifier: &parser::Identifier,
    ) -> Result<Box<dyn KyaObject>, KyaError> {
        if let Some(object) = self.context.get(&identifier.name) {
            return Ok(object.dup());
        }

        Err(KyaError::RuntimeError(format!(
            "Name '{}' is not defined",
            identifier.name
        )))
    }

    fn eval_function_call(
        &mut self,
        function_call: &parser::FunctionCall,
    ) -> Result<Box<dyn KyaObject>, KyaError> {
        let func = function_call.func.eval(self)?;
        let name = function_call.func.as_any().downcast_ref::<parser::Name>();
        let identifier = name
            .unwrap()
            .identifier
            .as_any()
            .downcast_ref::<parser::Identifier>();

        if let Some(func) = func.as_any().downcast_ref::<KyaRsFunction>() {
            let mut args = vec![];

            for arg in &function_call.arguments {
                args.push(arg.eval(self)?);
            }

            return func.call(&self.context, args);
        }

        Err(KyaError::RuntimeError(format!(
            "Function call failed: {}",
            identifier.unwrap().name
        )))
    }

    fn eval_string_literal(
        &mut self,
        string_literal: &parser::StringLiteral,
    ) -> Result<Box<dyn KyaObject>, KyaError> {
        Ok(Box::new(KyaString::new(string_literal.value.clone())))
    }
}
