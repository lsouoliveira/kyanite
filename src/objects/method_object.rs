use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef};
use crate::objects::instance_object::register_self;
use crate::objects::string_object::StringObject;
use std::cell::RefCell;
use std::rc::Rc;

pub struct MethodObject {
    pub ob_type: TypeRef,
    pub function: KyaObjectRef,
    pub instance_object: KyaObjectRef,
}

impl KyaObjectTrait for MethodObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn create_method_type() -> TypeRef {
    Type::as_ref(Type {
        name: "Method".to_string(),
        tp_repr: Some(method_tp_repr),
        tp_call: Some(method_tp_call),
        ..Default::default()
    })
}

pub fn method_tp_repr(
    interpreter: &mut Interpreter,
    callable: KyaObjectRef,
    _args: Vec<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.borrow();

    if let KyaObject::MethodObject(method_object) = &*object {
        let instance_type = method_object.instance_object.borrow().get_type()?;

        Ok(KyaObject::from_string_object(StringObject {
            ob_type: interpreter.get_type("String"),
            value: format!(
                "<bound method {} of {}>",
                instance_type.borrow().name,
                format!(
                    "<instance {} at {:p}>",
                    instance_type.borrow().name,
                    &*method_object.instance_object.borrow() as *const KyaObject
                )
            ),
        }))
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a method",
            object.get_type()?.borrow().name
        )))
    }
}

pub fn method_tp_call(
    interpreter: &mut Interpreter,
    callable: KyaObjectRef,
    args: Vec<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.borrow();

    if let KyaObject::MethodObject(method_object) = &*object {
        interpreter.push_frame();

        register_self(interpreter, method_object.instance_object.clone());

        let result = method_object.function.borrow().get_type()?.borrow().call(
            interpreter,
            method_object.function.clone(),
            args,
        );

        interpreter.pop_frame();

        result
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a method",
            object.get_type()?.borrow().name
        )))
    }
}
