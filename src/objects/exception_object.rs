use crate::bytecode::ComparisonOperator;
use crate::errors::Error;
use crate::interpreter::NONE_OBJECT;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef, BASE_TYPE};
use crate::objects::list_object::list_new;
use crate::objects::number_object::number_new;
use crate::objects::rs_function_object::rs_function_new;
use crate::objects::utils::{bool_to_bool_object, parse_arg, parse_receiver};
use once_cell::sync::Lazy;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

pub struct ExceptionObject {
    pub ob_type: TypeRef,
    pub message: KyaObjectRef,
}

impl KyaObjectTrait for ExceptionObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn exception_new(message: KyaObjectRef) -> KyaObjectRef {
    KyaObject::from_exception(ExceptionObject {
        ob_type: EXCEPTION_TYPE.clone(),
        message,
    })
}

pub fn exception_tp_new(
    _ob_type: TypeRef,
    args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let arg = parse_arg(args, 0, 1)?;

    Ok(exception_new(arg))
}

pub fn exception_tp_init(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    Ok(NONE_OBJECT.clone())
}

pub static EXCEPTION_TYPE: Lazy<TypeRef> = Lazy::new(|| {
    Type::as_ref(Type {
        ob_type: Some(BASE_TYPE.clone()),
        name: "Exception".to_string(),
        tp_new: Some(exception_tp_new),
        tp_init: Some(exception_tp_init),
        ..Default::default()
    })
});
