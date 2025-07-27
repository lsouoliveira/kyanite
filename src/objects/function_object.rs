use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::bytecode::CodeObject;
use crate::errors::Error;
use crate::interpreter::{eval_frame, Frame};
use crate::objects::base::{
    DictRef, KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef, BASE_TYPE,
};
use crate::objects::none_object::none_new;
use crate::objects::string_object::{StringObject, STRING_TYPE};

pub struct FunctionObject {
    pub ob_type: TypeRef,
    pub name: String,
    pub code: Arc<CodeObject>,
    pub globals: DictRef,
}

impl KyaObjectTrait for FunctionObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn function_repr(
    callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.lock().unwrap();

    if let KyaObject::FunctionObject(_) = &*object {
        Ok(KyaObject::from_string_object(StringObject {
            ob_type: STRING_TYPE.clone(),
            value: format!(
                "<function {} at {:p}>",
                object.get_type()?.lock().unwrap().name,
                &*object as *const KyaObject
            ),
        }))
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a function",
            object.get_type()?.lock().unwrap().name
        )))
    }
}

pub fn function_call(
    callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    if let KyaObject::FunctionObject(func) = &*callable.lock().unwrap() {
        if func.code.args.len() != args.len() {
            return Err(Error::RuntimeError(format!(
                "Function '{}' expects {} arguments, but got {}",
                func.name,
                func.code.args.len(),
                args.len()
            )));
        }

        let mut locals = HashMap::new();

        if let Some(receiver_obj) = receiver {
            locals.insert("self".to_string(), receiver_obj);
        }

        for (i, arg) in func.code.args.iter().enumerate() {
            locals.insert(arg.clone(), args[i].clone());
        }

        let mut frame_ref = Frame {
            locals: Arc::new(Mutex::new(locals)),
            globals: func.globals.clone(),
            code: func.code.clone(),
            pc: 0,
            stack: vec![],
            return_value: None,
        };

        eval_frame(&mut frame_ref)
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not callable",
            callable.lock().unwrap().get_type()?.lock().unwrap().name
        )))
    }
}

pub fn function_new(name: String, code: Arc<CodeObject>, globals: DictRef) -> KyaObjectRef {
    KyaObject::from_function_object(FunctionObject {
        ob_type: FUNCTION_TYPE.clone(),
        name,
        code,
        globals,
    })
}

pub static FUNCTION_TYPE: Lazy<TypeRef> = Lazy::new(|| {
    Type::as_ref(Type {
        ob_type: Some(BASE_TYPE.clone()),
        name: "Function".to_string(),
        tp_repr: Some(function_repr),
        tp_call: Some(function_call),
        ..Default::default()
    })
});
