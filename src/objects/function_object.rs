use crate::ast;
use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef};
use crate::objects::string_object::StringObject;

pub struct FunctionObject {
    pub ob_type: TypeRef,
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<Box<ast::ASTNode>>,
}

impl FunctionObject {
    pub fn new(
        ob_type: TypeRef,
        name: String,
        parameters: Vec<String>,
        body: Vec<Box<ast::ASTNode>>,
    ) -> Self {
        Self {
            ob_type,
            name,
            parameters,
            body,
        }
    }
}

impl KyaObjectTrait for FunctionObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn create_function_type(ob_type: TypeRef) -> TypeRef {
    Type::as_ref(Type {
        ob_type: Some(ob_type.clone()),
        name: "Function".to_string(),
        tp_repr: Some(function_repr),
        tp_call: Some(function_call),
        ..Default::default()
    })
}

pub fn function_repr(
    _interpreter: &mut Interpreter,
    callable: KyaObjectRef,
    _args: Vec<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.borrow();

    if let KyaObject::FunctionObject(_) = &*object {
        Ok(KyaObject::from_string_object(StringObject {
            ob_type: _interpreter.get_type("String"),
            value: format!(
                "<function {} at {:p}>",
                object.get_type()?.borrow().name,
                &*object as *const KyaObject
            ),
        }))
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a function",
            object.get_type()?.borrow().name
        )))
    }
}

pub fn function_call(
    interpreter: &mut Interpreter,
    callable: KyaObjectRef,
    args: Vec<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.borrow();

    if let KyaObject::FunctionObject(func) = &*object {
        if func.parameters.len() != args.len() {
            return Err(Error::RuntimeError(format!(
                "Function '{}' expects {} arguments, but got {}",
                func.name,
                func.parameters.len(),
                args.len()
            )));
        }

        interpreter.push_frame();

        for (param, arg) in func.parameters.iter().zip(args) {
            interpreter.register(param, arg);
        }

        let mut result = interpreter.resolve("None");

        for statement in &func.body {
            result = statement.eval(interpreter);
        }

        interpreter.pop_frame();

        result
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not callable",
            object.get_type()?.borrow().name
        )))
    }
}
