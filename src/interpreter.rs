use crate::ast;
use crate::builtins::{kya_globals, kya_input, kya_print};
use crate::builtins_::bool::kya_bool_new;
use crate::builtins_::modules::math;
use crate::builtins_::number::kya_number_new;
use crate::builtins_::string::{instantiate_string, kya_string_new};
use crate::errors::Error;
use crate::lexer::Lexer;
use crate::lexer::TokenType;
use crate::objects::{
    Context, KyaClass, KyaFrame, KyaFunction, KyaMethod, KyaModule, KyaNone, KyaObject, KyaRsClass,
    KyaRsFunction,
};
use crate::parser;
use crate::visitor::Evaluator;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

pub struct Interpreter {
    root: PathBuf,
    input: String,
    pub context: Context,
    pub frames: Vec<Rc<RefCell<KyaFrame>>>,
    builtin_modules: HashMap<String, KyaObject>,
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
        String::from("input"),
        Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
            String::from("input"),
            kya_input,
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
        Rc::new(KyaObject::RsClass(KyaRsClass::new(
            String::from("String"),
            vec![String::from("value")],
            instantiate_string,
        ))),
    );

    context.register(String::from("true"), kya_bool_new(true).unwrap());
    context.register(String::from("false"), kya_bool_new(false).unwrap());
}

fn import_module(root: &PathBuf, module_name: &str) -> Result<KyaObject, Error> {
    let module_path = root.join(format!("{}.k", module_name));
    let input = fs::read_to_string(&module_path)
        .map_err(|_| Error::RuntimeError(format!("Could not read module: {}", module_name)))?;
    let mut interpreter = Interpreter::new(input, root.to_string_lossy().to_string());

    interpreter.evaluate()?;

    Ok(KyaObject::Module(KyaModule::new(
        module_name.to_string(),
        interpreter.context.clone(),
    )))
}

impl Interpreter {
    pub fn new(input: String, root: String) -> Self {
        let mut context = Context::new();

        setup_builtins(&mut context);

        let builtin_modules = vec![("math", math::pack_module())];
        let root_path = PathBuf::from(root);

        Interpreter {
            root: root_path,
            input,
            context,
            frames: vec![],
            builtin_modules: builtin_modules
                .into_iter()
                .map(|(name, module)| (name.to_string(), module))
                .collect(),
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
            if let KyaObject::Module(module) = object.as_ref() {
                if let Some(module_object) = module.resolve(name) {
                    return Some(module_object);
                }
            }

            return Some(object.clone());
        }

        if let Some(frame) = self.frames.last() {
            if let Some(object) = frame.borrow().locals.get(name) {
                return Some(object.clone());
            }
        }

        None
    }

    pub fn get_self(&self) -> Result<Rc<KyaObject>, Error> {
        self.resolve("self")
            .ok_or_else(|| Error::RuntimeError("Undefined identifier: self".to_string()))
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

    pub fn true_object(&self) -> Rc<KyaObject> {
        self.context.get("true").unwrap().clone()
    }

    pub fn false_object(&self) -> Rc<KyaObject> {
        self.context.get("false").unwrap().clone()
    }

    fn is_true(&self, object: &KyaObject) -> bool {
        match object {
            KyaObject::Bool(value) => *value,
            KyaObject::Number(value) => *value != 0.0,
            KyaObject::String(value) => !value.value.is_empty(),
            KyaObject::InstanceObject(instance) => {
                if instance.name() == "Bool" {
                    instance.get_bool_attribute("__value__").unwrap()
                } else if instance.name() == "String" {
                    !instance
                        .get_string_attribute("__value__")
                        .unwrap()
                        .is_empty()
                } else {
                    true
                }
            }
            KyaObject::None(_) => false,
            _ => true,
        }
    }

    pub fn eval_body(&mut self, body: &[Box<ast::ASTNode>]) -> Result<Rc<KyaObject>, Error> {
        let mut result = Rc::new(KyaObject::None(KyaNone {}));

        for statement in body {
            result = statement.eval(self)?;
        }

        Ok(result)
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

        Ok(callee.call(self, args)?)
    }

    fn eval_assignment(&mut self, assignment: &ast::Assignment) -> Result<Rc<KyaObject>, Error> {
        let value = assignment.value.eval(self)?;

        if let ast::ASTNode::Identifier(identifier) = &*assignment.name {
            if identifier.name == "true" || identifier.name == "false" {
                return Err(Error::RuntimeError(format!(
                    "Cannot assign to reserved keyword: {}",
                    identifier.name
                )));
            }

            self.register_local(identifier.name.clone(), value.clone())?;
        } else if let ast::ASTNode::Attribute(attribute) = &*assignment.name {
            let instance = attribute.name.eval(self)?;

            if let KyaObject::InstanceObject(instance_object) = instance.as_ref() {
                instance_object.set_attribute(attribute.value.clone(), value.clone());
            }
        } else {
            return Err(Error::RuntimeError(format!(
                "Invalid assignment target: {:?}",
                assignment.name
            )));
        }

        Ok(value)
    }

    fn eval_number_literal(&mut self, number_literal: &f64) -> Result<Rc<KyaObject>, Error> {
        Ok(kya_number_new(*number_literal)?)
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
                    return Ok(Rc::new(KyaObject::Method(KyaMethod {
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
        } else if let KyaObject::Module(module) = name.as_ref() {
            if let Some(object) = module.resolve(attribute.value.as_str()) {
                return Ok(object);
            } else {
                return Err(Error::RuntimeError(format!(
                    "Undefined attribute in module: {}",
                    attribute.value
                )));
            }
        }

        Err(Error::RuntimeError(format!(
            "Invalid attribute assignment: {}",
            attribute.value
        )))
    }

    fn eval_compare(&mut self, compare: &ast::Compare) -> Result<Rc<KyaObject>, Error> {
        let left = compare.left.eval(self)?;
        let right = compare.right.eval(self)?;

        return left.get_attribute("__eq__").call(self, vec![right.clone()]);
    }

    fn eval_if(&mut self, if_node: &ast::If) -> Result<Rc<KyaObject>, Error> {
        let test = if_node.test.eval(self)?;

        if self.is_true(&test) {
            return self.eval_body(&if_node.body);
        }

        Ok(Rc::new(KyaObject::None(KyaNone {})))
    }

    fn eval_import(&mut self, import: &ast::Import) -> Result<Rc<KyaObject>, Error> {
        if let Some(module) = self.builtin_modules.get(&import.name) {
            self.context
                .register(import.name.clone(), Rc::new(module.clone()));

            return Ok(Rc::new(KyaObject::None(KyaNone {})));
        }

        let module = import_module(&self.root, &import.name)?;

        self.context.register(import.name.clone(), Rc::new(module));

        return Ok(Rc::new(KyaObject::None(KyaNone {})));
    }

    fn eval_bin_op(&mut self, bin_op: &ast::BinOp) -> Result<Rc<KyaObject>, Error> {
        let left = bin_op.left.eval(self)?;
        let right = bin_op.right.eval(self)?;

        if let KyaObject::InstanceObject(_) = left.as_ref() {
            if let KyaObject::InstanceObject(_) = right.as_ref() {
                match bin_op.operator {
                    TokenType::Plus => {
                        let args = vec![right];

                        return left.get_attribute("__add__").call(self, args);
                    }
                    _ => {
                        return Err(Error::RuntimeError(format!(
                            "Unsupported binary operator: {:?}",
                            bin_op.operator
                        )));
                    }
                }
            }
        }

        Err(Error::RuntimeError(format!(
            "Invalid left operand for binary operation: {}",
            left.repr()
        )))
    }

    fn eval_unary_op(&mut self, unary_op: &ast::UnaryOp) -> Result<Rc<KyaObject>, Error> {
        let operand = unary_op.operand.eval(self)?;

        match unary_op.operator {
            TokenType::Minus => {
                return operand.get_attribute("__neg__").call(self, vec![]);
            }
            _ => {
                return Err(Error::RuntimeError(format!(
                    "Unsupported unary operator: {:?}",
                    unary_op.operator
                )));
            }
        }
    }
}
