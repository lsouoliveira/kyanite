use crate::ast;
use crate::builtins::methods::kya_print;
use crate::errors::Error;
use crate::lexer::Lexer;
use crate::objects::class_object::{create_class_type, ClassObject};
use crate::objects::function_object::{create_function_type, FunctionObject};
use crate::objects::method_object::create_method_type;
use crate::objects::modules::sockets::functions::kya_socket;
use crate::objects::modules::sockets::socket_object::create_socket_type;
use crate::objects::none_object::{create_none_type, none_new};
use crate::objects::number_object::{create_number_type, NumberObject};
use crate::objects::rs_function_object::{create_rs_function_type, RsFunctionObject};
use crate::objects::string_object::{create_string_type, StringObject};
use crate::objects::utils::{create_rs_function_object, string_object_to_string};
use crate::parser;
use crate::visitor::Evaluator;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use crate::objects::base::{create_type_type, DictRef, KyaObject, KyaObjectRef, Type, TypeRef};

pub static NONE_TYPE: &str = "None";
pub static STRING_TYPE: &str = "String";
pub static RS_FUNCTION_TYPE: &str = "RsFunction";
pub static FUNCTION_TYPE: &str = "Function";
pub static NUMBER_TYPE: &str = "Number";
pub static METHOD_TYPE: &str = "Method";
pub static SOCKET_TYPE: &str = "Socket";

type FrameRef = Rc<RefCell<Frame>>;

pub struct Interpreter {
    root: PathBuf,
    input: String,

    globals: HashMap<String, KyaObjectRef>,
    frames: Vec<FrameRef>,

    types: HashMap<String, TypeRef>,
}

// fn import_module(root: &PathBuf, module_name: &str) -> Result<KyaObject, Error> {
//     let module_path = root.join(format!("{}.k", module_name));
//     let input = fs::read_to_string(&module_path)
//         .map_err(|_| Error::RuntimeError(format!("Could not read module: {}", module_name)))?;
//     let mut interpreter = Interpreter::new(input, root.to_string_lossy().to_string());
//
//     interpreter.evaluate()?;
//
//     Ok(KyaObject::Module(KyaModule {
//         name: module_name.to_string(),
//         objects: RefCell::new(interpreter.context.clone()),
//     }))
// }

pub struct Frame {
    pub locals: DictRef,
}

impl Interpreter {
    pub fn new(input: String, root: String) -> Self {
        let root_path = PathBuf::from(root);

        Interpreter {
            root: root_path,
            input,
            globals: HashMap::new(),
            types: HashMap::new(),
            frames: vec![Rc::new(RefCell::new(Frame {
                locals: Rc::new(RefCell::new(HashMap::new())),
            }))],
        }
    }

    pub fn init(&mut self) {
        self.register_types();
        self.register_builtins();
    }

    pub fn current_frame(&self) -> FrameRef {
        self.frames.last().unwrap().clone()
    }

    pub fn print_frames(&self) {
        for (i, frame) in self.frames.iter().enumerate() {
            for (name, object) in frame.borrow().locals.borrow().iter() {
                println!("Frame {}: {}", i, name,);
            }
        }
    }

    pub fn push_frame(&mut self) {
        self.frames.push(Rc::new(RefCell::new(Frame {
            locals: Rc::new(RefCell::new(HashMap::new())),
        })));
    }

    pub fn pop_frame(&mut self) {
        if self.frames.len() > 1 {
            self.frames.pop();
        } else {
            panic!("Cannot pop the global frame");
        }
    }

    pub fn register_types(&mut self) {
        let type_type = create_type_type();
        let none_type = create_none_type(type_type.clone());

        self.types.insert("Type".to_string(), type_type.clone());
        self.types.insert("None".to_string(), none_type.clone());
        self.types.insert(
            STRING_TYPE.to_string(),
            create_string_type(type_type.clone()),
        );
        self.types.insert(
            RS_FUNCTION_TYPE.to_string(),
            create_rs_function_type(type_type.clone()),
        );
        self.types.insert(
            FUNCTION_TYPE.to_string(),
            create_function_type(type_type.clone()),
        );
        self.types.insert(
            NUMBER_TYPE.to_string(),
            create_number_type(type_type.clone()),
        );
        self.types
            .insert(METHOD_TYPE.to_string(), create_method_type());

        let socket_type = create_socket_type(self, type_type.clone());

        self.types.insert(SOCKET_TYPE.to_string(), socket_type);
    }

    pub fn register_builtins(&mut self) {
        let none_object = none_new(self, self.get_type(NONE_TYPE), vec![]).unwrap();

        self.globals
            .insert(NONE_TYPE.to_string(), none_object.clone());

        let kya_print_function_object = create_rs_function_object(self, kya_print);

        self.globals
            .insert("print".to_string(), kya_print_function_object.clone());

        let kya_socket_function_object = create_rs_function_object(self, kya_socket);

        self.globals
            .insert("socket".to_string(), kya_socket_function_object.clone());
    }

    pub fn get_type(&self, name: &str) -> TypeRef {
        self.types.get(name).cloned().unwrap()
    }

    pub fn resolve(&self, name: &str) -> Result<KyaObjectRef, Error> {
        for frame in self.frames.iter().rev() {
            if let Some(object) = frame.borrow().locals.borrow().get(name) {
                return Ok(object.clone());
            }
        }

        if let Some(object) = self.globals.get(name) {
            return Ok(object.clone());
        }

        Err(Error::RuntimeError(format!(
            "name '{}' is not defined",
            name
        )))
    }

    pub fn resolve_self(&self) -> Result<KyaObjectRef, Error> {
        self.resolve("self")
    }

    pub fn register(&mut self, name: &str, object: KyaObjectRef) {
        self.current_frame()
            .borrow_mut()
            .locals
            .borrow_mut()
            .insert(name.to_string(), object);
    }

    pub fn evaluate(&mut self) -> Result<KyaObjectRef, Error> {
        let lexer = Lexer::new(self.input.clone());
        let mut parser = parser::Parser::new(lexer);

        match parser.parse() {
            Ok(module) => Ok(module.eval(self)?),
            Err(e) => Err(Error::RuntimeError(format!("Parse error: {}", e))),
        }
    }

    pub fn get_none(&self) -> KyaObjectRef {
        self.resolve(NONE_TYPE).unwrap()
    }
}

impl Evaluator for Interpreter {
    fn eval_module(&mut self, module: &ast::Module) -> Result<KyaObjectRef, Error> {
        let mut result = self.resolve(NONE_TYPE).unwrap();

        for statement in &module.statements {
            result = statement.eval(self)?;
        }

        Ok(result)
    }

    fn eval_identifier(&mut self, identifier: &ast::Identifier) -> Result<KyaObjectRef, Error> {
        self.resolve(&identifier.name)
    }

    fn eval_string_literal(&mut self, string_literal: &str) -> Result<KyaObjectRef, Error> {
        Ok(KyaObject::from_string_object(StringObject {
            ob_type: self.get_type(STRING_TYPE),
            value: string_literal.to_string(),
        }))
    }

    fn eval_method_call(&mut self, method_call: &ast::MethodCall) -> Result<KyaObjectRef, Error> {
        self.push_frame();

        let name = method_call.name.eval(self)?;
        let args = method_call
            .arguments
            .iter()
            .map(|arg| arg.eval(self))
            .collect::<Result<Vec<KyaObjectRef>, Error>>()?;

        let result = name
            .borrow()
            .get_type()?
            .borrow()
            .call(self, name.clone(), args)?;

        self.pop_frame();

        Ok(result)
    }

    fn eval_assignment(&mut self, assignment: &ast::Assignment) -> Result<KyaObjectRef, Error> {
        let value = assignment.value.eval(self)?;

        if let ast::ASTNode::Identifier(identifier) = &*assignment.name {
            self.register(identifier.name.as_str(), value.clone());
        } else if let ast::ASTNode::Attribute(attribute) = &*assignment.name {
            let object = attribute.name.eval(self)?;

            object.borrow().get_type()?.borrow().set_attr(
                self,
                object.clone(),
                attribute.value.clone(),
                value.clone(),
            )?;
        } else {
            return Err(Error::RuntimeError("Invalid assignment target".to_string()));
        }

        Ok(value)
    }

    fn eval_number_literal(&mut self, number_literal: &f64) -> Result<KyaObjectRef, Error> {
        Ok(KyaObject::from_number_object(NumberObject {
            ob_type: self.get_type(NUMBER_TYPE),
            value: *number_literal,
        }))
    }

    fn eval_method_def(&mut self, method_def: &ast::MethodDef) -> Result<KyaObjectRef, Error> {
        let mut parameters = vec![];

        for param in &method_def.parameters {
            if let ast::ASTNode::Identifier(identifier) = &**param {
                parameters.push(identifier.name.clone());
            } else {
                return Err(Error::RuntimeError("Invalid parameter name".to_string()));
            }
        }

        let object = KyaObject::from_function_object(FunctionObject {
            ob_type: self.get_type(FUNCTION_TYPE),
            name: method_def.name.clone(),
            parameters,
            body: method_def.body.clone(),
        });

        self.register(&method_def.name, object.clone());

        Ok(self.resolve(NONE_TYPE).unwrap())
    }

    fn eval_class_def(&mut self, class_def: &ast::ClassDef) -> Result<KyaObjectRef, Error> {
        self.push_frame();

        for stmt in &class_def.body {
            stmt.eval(self)?;
        }

        let class_type = create_class_type(
            self,
            class_def.name.clone(),
            self.current_frame().borrow().locals.clone(),
        );

        let object = KyaObject::from_class_object(ClassObject {
            ob_type: class_type.clone(),
        });

        self.pop_frame();

        self.register(&class_def.name, object.clone());

        Ok(self.resolve(NONE_TYPE).unwrap())
    }

    fn eval_attribute(&mut self, attribute: &ast::Attribute) -> Result<KyaObjectRef, Error> {
        if let ast::ASTNode::Identifier(identifier) = &*attribute.name {
            let object = attribute.name.eval(self)?;

            return object.borrow().get_type()?.borrow().get_attr(
                self,
                object.clone(),
                attribute.value.clone(),
            );
        }

        Err(Error::RuntimeError("Invalid attribute access".to_string()))
    }

    fn eval_compare(&mut self, compare: &ast::Compare) -> Result<KyaObjectRef, Error> {
        Ok(self.resolve(NONE_TYPE).unwrap())
    }

    fn eval_if(&mut self, if_node: &ast::If) -> Result<KyaObjectRef, Error> {
        Ok(self.resolve(NONE_TYPE).unwrap())
    }

    fn eval_import(&mut self, import: &ast::Import) -> Result<KyaObjectRef, Error> {
        Ok(self.resolve(NONE_TYPE).unwrap())
    }

    fn eval_bin_op(&mut self, bin_op: &ast::BinOp) -> Result<KyaObjectRef, Error> {
        Ok(self.resolve(NONE_TYPE).unwrap())
    }

    fn eval_unary_op(&mut self, unary_op: &ast::UnaryOp) -> Result<KyaObjectRef, Error> {
        Ok(self.resolve(NONE_TYPE).unwrap())
    }
}
