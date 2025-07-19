use crate::errors::Error;
use crate::interpreter::{self, Interpreter, STRING_TYPE};
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef};
use crate::objects::number_object::NumberObject;
use crate::objects::rs_function_object::RsFunctionObject;
use crate::objects::string_object::StringObject;
use crate::objects::utils::{parse_arg, parse_receiver};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct BytesObject {
    pub ob_type: TypeRef,
    pub value: Vec<u8>,
}

impl KyaObjectTrait for BytesObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn create_bytes_type(ob_type: TypeRef, rs_function_type: TypeRef) -> TypeRef {
    let dict = Rc::new(RefCell::new(HashMap::new()));

    dict.borrow_mut().insert(
        "length".to_string(),
        KyaObject::from_rs_function_object(RsFunctionObject::new(
            rs_function_type.clone(),
            bytes_length,
        )),
    );

    Type::as_ref(Type {
        ob_type: Some(ob_type.clone()),
        name: "Bytes".to_string(),
        tp_repr: Some(bytes_tp_repr),
        sq_len: Some(bytes_sq_len),
        dict: dict.clone(),
        ..Default::default()
    })
}

pub fn bytes_tp_repr(
    interpreter: &mut Interpreter,
    callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.borrow();

    if let KyaObject::BytesObject(obj) = &*object {
        Ok(KyaObject::from_string_object(StringObject {
            ob_type: interpreter.get_type(STRING_TYPE),
            value: format!("b'{}'", String::from_utf8_lossy(&obj.value)),
        }))
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a bytes object.",
            object.get_type()?.borrow().name
        )))
    }
}

pub fn bytes_sq_len(_interpreter: &mut Interpreter, object: KyaObjectRef) -> Result<usize, Error> {
    if let KyaObject::BytesObject(obj) = &*object.borrow() {
        Ok(obj.value.len())
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a bytes object.",
            object.borrow().get_type()?.borrow().name
        )))
    }
}

pub fn bytes_length(
    interpreter: &mut Interpreter,
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let instance = parse_receiver(&receiver)?;

    if let KyaObject::BytesObject(obj) = &*instance.borrow() {
        Ok(KyaObject::from_number_object(NumberObject {
            ob_type: interpreter.get_type("Number"),
            value: obj.value.len() as f64,
        }))
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a bytes object.",
            instance.borrow().get_type()?.borrow().name
        )))
    }
}
