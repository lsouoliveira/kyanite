use crate::bytecode::ComparisonOperator;
use crate::errors::Error;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef, BASE_TYPE};
use crate::objects::string_object::{StringObject, STRING_TYPE};

use once_cell::sync::Lazy;
use std::sync::Arc;

pub struct BoolObject {
    pub ob_type: TypeRef,
    pub value: bool,
}

impl KyaObjectTrait for BoolObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn bool_tp_repr(
    callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.lock().unwrap();

    if let KyaObject::BoolObject(obj) = &*object {
        let repr = if obj.value {
            "true".to_string()
        } else {
            "false".to_string()
        };

        Ok(KyaObject::from_string_object(StringObject {
            ob_type: STRING_TYPE.clone(),
            value: repr,
        }))
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a string",
            object.get_type()?.lock().unwrap().name
        )))
    }
}

pub fn bool_nb_bool(object: KyaObjectRef) -> Result<f64, Error> {
    if let KyaObject::BoolObject(obj) = &*object.lock().unwrap() {
        Ok(if obj.value { 1.0 } else { 0.0 })
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a bool",
            object.lock().unwrap().get_type()?.lock().unwrap().name
        )))
    }
}

pub fn bool_new(value: bool) -> KyaObjectRef {
    KyaObject::from_bool_object(BoolObject {
        ob_type: BOOL_TYPE.clone(),
        value,
    })
}

pub static BOOL_TYPE: Lazy<TypeRef> = Lazy::new(|| {
    Type::as_ref(Type {
        ob_type: Some(BASE_TYPE.clone()),
        name: "Bool".to_string(),
        tp_repr: Some(bool_tp_repr),
        nb_bool: Some(bool_nb_bool),
        ..Default::default()
    })
});
