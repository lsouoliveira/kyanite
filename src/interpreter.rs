use crate::ast;
use crate::builtins::kya_print;
use crate::errors::Error;
use crate::lexer::Lexer;
use crate::objects::{
    Context, KyaFunction, KyaFunctionFrame, KyaNone, KyaObject, KyaRsFunction, KyaString,
};
use crate::parser;
use crate::visitor::Evaluator;
use std::rc::Rc;

pub struct Interpreter {
    input: String,
    context: Context,
    function_frames: Vec<Rc<KyaFunctionFrame>>,
}

fn setup_builtins(context: &mut Context) {
    context.register(
        String::from("print"),
        Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
            String::from("print"),
            kya_print,
        ))),
    );
}

impl Interpreter {
    pub fn new(input: String) -> Self {
        let mut context = Context::new(None);

        setup_builtins(&mut context);

        Interpreter {
            input,
            context,
            function_frames: vec![],
        }
    }

    pub fn evaluate(&mut self) -> Result<(), Error> {
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
    fn eval_module(&mut self, module: &ast::Module) -> Result<Rc<KyaObject>, Error> {
        for statement in &module.statements {
            statement.eval(self)?;
        }

        Ok(Rc::new(KyaObject::None(KyaNone {})))
    }

    fn eval_identifier(&mut self, identifier: &ast::Identifier) -> Result<Rc<KyaObject>, Error> {
        if let Some(object) = self.context.get(&identifier.name) {
            Ok(object.clone())
        } else {
            Err(Error::RuntimeError(format!(
                "Undefined identifier: {}",
                identifier.name
            )))
        }
    }

    fn eval_string_literal(&mut self, string_literal: &str) -> Result<Rc<KyaObject>, Error> {
        Ok(Rc::new(KyaObject::String(KyaString {
            value: string_literal.to_string(),
        })))
    }

    fn eval_method_call(&mut self, method_call: &ast::MethodCall) -> Result<Rc<KyaObject>, Error> {
        let callee = method_call.name.eval(self)?;
        let args: Vec<Rc<KyaObject>> = method_call
            .arguments
            .iter()
            .map(|arg| arg.eval(self))
            .collect::<Result<Vec<_>, _>>()?;

        if let KyaObject::RsFunction(func) = callee.as_ref() {
            return func.call(&self.context, args);
        } else if let KyaObject::Function(func) = callee.as_ref() {
            let frame = Rc::new(KyaFunctionFrame::new(func.clone()));

            self.function_frames.push(frame.clone());

            let mut result = Rc::new(KyaObject::None(KyaNone {}));

            for stmt in &frame.function.body {
                result = if let Ok(returned_value) = stmt.eval(self) {
                    returned_value
                } else {
                    return Err(Error::RuntimeError(format!(
                        "Error evaluating method: {}",
                        func.name
                    )));
                };
            }

            self.function_frames.pop();

            return Ok(result);
        }

        if let ast::ASTNode::Identifier(identifier) = &*method_call.name {
            return Err(Error::RuntimeError(format!(
                "Undefined method: {}",
                identifier.name
            )));
        }

        Err(Error::RuntimeError(format!("Unexpected method call",)))
    }

    fn eval_assignment(&mut self, assignment: &ast::Assignment) -> Result<Rc<KyaObject>, Error> {
        let value = assignment.value.eval(self)?;

        self.context
            .register(assignment.name.clone(), value.clone());

        Ok(value)
    }

    fn eval_number_literal(&mut self, number_literal: &f64) -> Result<Rc<KyaObject>, Error> {
        Ok(Rc::new(KyaObject::Number(*number_literal)))
    }

    fn eval_method_def(&mut self, method_def: &ast::MethodDef) -> Result<Rc<KyaObject>, Error> {
        let function = Rc::new(KyaObject::Function(KyaFunction::new(
            method_def.name.clone(),
            method_def.body.clone(),
        )));

        self.context
            .register(method_def.name.clone(), function.clone());

        Ok(function)
    }
}
