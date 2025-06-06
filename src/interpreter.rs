use crate::ast;
use crate::builtins::{kya_globals, kya_print, kya_string_length, kya_string_new, kya_string_repr};
use crate::errors::Error;
use crate::lexer::Lexer;
use crate::objects::{
    Context, KyaClass, KyaFrame, KyaFunction, KyaInstanceObject, KyaMethod, KyaNone, KyaObject,
    KyaRsFunction, KyaRsMethod, KyaString,
};
use crate::parser;
use crate::visitor::Evaluator;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    input: String,
    pub context: Context,
    frames: Vec<Rc<RefCell<KyaFrame>>>,
}

fn setup_builtins(context: &mut Context) {
    context.register(
        String::from("print"),
        Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
            String::from("print"),
            kya_print,
        ))),
    );

    context.register(
        String::from("__globals__"),
        Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
            String::from("__globals__"),
            kya_globals,
        ))),
    );

    context.register(
        String::from("String"),
        Rc::new(KyaObject::Class(KyaClass::new(
            String::from("String"),
            vec![],
        ))),
    );
}

impl Interpreter {
    pub fn new(input: String) -> Self {
        let mut context = Context::new();

        setup_builtins(&mut context);

        Interpreter {
            input,
            context,
            frames: vec![],
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

    pub fn resolve(&self, name: &str) -> Option<Rc<KyaObject>> {
        if let Some(object) = self.context.get(name) {
            return Some(object.clone());
        }

        if let Some(frame) = self.frames.last() {
            if let Some(object) = frame.borrow().locals.get(name) {
                return Some(object.clone());
            }
        }

        None
    }

    fn register_local(
        &mut self,
        name: String,
        object: Rc<KyaObject>,
    ) -> Result<Rc<KyaObject>, Error> {
        if let Some(frame) = self.frames.last() {
            frame
                .borrow_mut()
                .locals
                .register(name.clone(), object.clone());
        } else {
            self.context.register(name, object.clone());
        }

        Ok(object)
    }

    pub fn call(
        &mut self,
        callee: Rc<KyaObject>,
        args: Vec<Rc<KyaObject>>,
    ) -> Result<Rc<KyaObject>, Error> {
        if let KyaObject::RsFunction(func) = callee.as_ref() {
            return func.call(self, args);
        } else if let KyaObject::Function(func) = callee.as_ref() {
            let frame = Rc::new(RefCell::new(KyaFrame::new()));

            if func.parameters.len() != args.len() {
                return Err(Error::RuntimeError(format!(
                    "{}() takes {} argument(s) but {} were given",
                    func.name,
                    func.parameters.len(),
                    args.len()
                )));
            }

            for (i, param) in func.parameters.iter().enumerate() {
                let arg = args[i].clone();
                frame.borrow_mut().locals.register(param.clone(), arg);
            }

            self.frames.push(frame);

            let mut result = Rc::new(KyaObject::None(KyaNone {}));

            for stmt in &func.body {
                result = stmt.eval(self)?;
            }

            self.frames.pop();

            return Ok(result);
        } else if let KyaObject::Class(class) = callee.as_ref() {
            let frame = Rc::new(RefCell::new(KyaFrame::new()));

            self.frames.push(frame.clone());

            class.body.iter().for_each(|stmt| {
                stmt.eval(self).unwrap();
            });

            self.frames.pop();

            return Ok(Rc::new(KyaObject::InstanceObject(KyaInstanceObject::new(
                frame.borrow().locals.clone(),
            ))));
        } else if let KyaObject::Method(method) = callee.as_ref() {
            if let KyaObject::Function(func) = method.function.as_ref() {
                let frame = Rc::new(RefCell::new(KyaFrame::new()));

                frame
                    .borrow_mut()
                    .locals
                    .register(String::from("self"), method.instance.clone());

                if func.parameters.len() != args.len() {
                    return Err(Error::RuntimeError(format!(
                        "{}() takes {} argument(s) but {} were given",
                        func.name,
                        func.parameters.len(),
                        args.len()
                    )));
                }

                for (i, param) in func.parameters.iter().enumerate() {
                    let arg = args[i].clone();
                    frame.borrow_mut().locals.register(param.clone(), arg);
                }

                self.frames.push(frame);

                let mut result = Rc::new(KyaObject::None(KyaNone {}));

                for stmt in &func.body {
                    result = stmt.eval(self)?;
                }

                self.frames.pop();

                return Ok(result);
            }
        } else if let KyaObject::RsMethod(method) = callee.as_ref() {
            if let KyaObject::RsFunction(func) = method.function.as_ref() {
                let frame = Rc::new(RefCell::new(KyaFrame::new()));

                frame
                    .borrow_mut()
                    .locals
                    .register(String::from("self"), method.instance.clone());

                self.frames.push(frame);
                let result = func.call(self, args)?;
                self.frames.pop();

                return Ok(result);
            }
        }

        Err(Error::RuntimeError(format!(
            "Cannot call non-function object: {}",
            callee.repr()
        )))
    }

    pub fn call_instance_method(
        &mut self,
        instance_object: Rc<KyaObject>,
        method_name: &str,
        args: Vec<Rc<KyaObject>>,
    ) -> Result<Rc<KyaObject>, Error> {
        if let KyaObject::InstanceObject(instance) = instance_object.as_ref() {
            let method = instance.get_attribute(method_name);

            if let Some(method) = method {
                let method = if let KyaObject::Function(_) = method.as_ref() {
                    Rc::new(KyaObject::Method(KyaMethod {
                        function: method.clone(),
                        instance: instance_object.clone(),
                    }))
                } else if let KyaObject::RsFunction(_) = method.as_ref() {
                    Rc::new(KyaObject::RsMethod(KyaRsMethod {
                        function: method.clone(),
                        instance: instance_object.clone(),
                    }))
                } else {
                    Err(Error::RuntimeError(format!(
                        "Invalid method type: {}",
                        method.repr()
                    )))?
                };

                return Ok(self.call(method, args)?);
            }
        }

        Err(Error::RuntimeError(format!(
            "Undefined method: {}",
            method_name
        )))
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
        let identifier = if let Some(object) = self.resolve(&identifier.name) {
            object.clone()
        } else {
            return Err(Error::RuntimeError(format!(
                "Undefined identifier: {}",
                identifier.name
            )));
        };

        Ok(identifier)
    }

    fn eval_string_literal(&mut self, string_literal: &str) -> Result<Rc<KyaObject>, Error> {
        Ok(kya_string_new(string_literal)?)
    }

    fn eval_method_call(&mut self, method_call: &ast::MethodCall) -> Result<Rc<KyaObject>, Error> {
        let callee = method_call.name.eval(self)?;
        let args: Vec<Rc<KyaObject>> = method_call
            .arguments
            .iter()
            .map(|arg| arg.eval(self))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(self.call(callee.clone(), args)?)
    }

    fn eval_assignment(&mut self, assignment: &ast::Assignment) -> Result<Rc<KyaObject>, Error> {
        let value = assignment.value.eval(self)?;

        if let ast::ASTNode::Identifier(identifier) = &*assignment.name {
            self.register_local(identifier.name.clone(), value.clone())?;
        } else {
            return Err(Error::RuntimeError(format!(
                "Invalid assignment target: {:?}",
                assignment.name
            )));
        }

        Ok(value)
    }

    fn eval_number_literal(&mut self, number_literal: &f64) -> Result<Rc<KyaObject>, Error> {
        Ok(Rc::new(KyaObject::Number(*number_literal)))
    }

    fn eval_method_def(&mut self, method_def: &ast::MethodDef) -> Result<Rc<KyaObject>, Error> {
        let mut parameters = vec![];

        for param in &method_def.parameters {
            if let ast::ASTNode::Identifier(identifier) = &**param {
                parameters.push(identifier.name.clone());
            } else {
                return Err(Error::RuntimeError(format!(
                    "Invalid parameter: {:?}",
                    param
                )));
            }
        }

        let function = Rc::new(KyaObject::Function(KyaFunction::new(
            method_def.name.clone(),
            parameters,
            method_def.body.clone(),
        )));

        self.register_local(method_def.name.clone(), function.clone())?;

        Ok(function)
    }

    fn eval_class_def(&mut self, class_def: &ast::ClassDef) -> Result<Rc<KyaObject>, Error> {
        let class = Rc::new(KyaObject::Class(KyaClass::new(
            class_def.name.clone(),
            class_def.body.clone(),
        )));

        self.register_local(class_def.name.clone(), class.clone())?;

        Ok(class)
    }

    fn eval_attribute(&mut self, attribute: &ast::Attribute) -> Result<Rc<KyaObject>, Error> {
        let name = attribute.name.eval(self)?;

        if let KyaObject::InstanceObject(instance_object) = name.as_ref() {
            if let Some(object) = instance_object.get_attribute(attribute.value.as_str()) {
                if let KyaObject::Function(_) = object.as_ref() {
                    return Ok(Rc::new(KyaObject::Method(KyaMethod {
                        function: object.clone(),
                        instance: name.clone(),
                    })));
                } else if let KyaObject::RsFunction(_) = object.as_ref() {
                    return Ok(Rc::new(KyaObject::RsMethod(KyaRsMethod {
                        function: object.clone(),
                        instance: name.clone(),
                    })));
                } else {
                    return Ok(object);
                }
            } else {
                return Err(Error::RuntimeError(format!(
                    "Undefined attribute: {}",
                    attribute.value
                )));
            }
        }

        Err(Error::RuntimeError(format!(
            "Invalid attribute assignment: {}",
            attribute.value
        )))
    }
}
