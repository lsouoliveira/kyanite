use crate::bytecode::ComparisonOperator;
use crate::errors::Error;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef, BASE_TYPE};
use crate::objects::bool_object::bool_new;
use crate::objects::string_object::string_new;

use once_cell::sync::Lazy;
use std::sync::Arc;

pub struct NoneObject {
    ob_type: TypeRef,
}

impl KyaObjectTrait for NoneObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn none_new() -> Result<KyaObjectRef, Error> {
    Ok(KyaObject::from_none_object(NoneObject {
        ob_type: NONE_TYPE.clone(),
    }))
}

pub fn none_repr(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    Ok(string_new("None"))
}

pub fn none_tp_compare(
    obj1: KyaObjectRef,
    obj2: KyaObjectRef,
    _operator: ComparisonOperator,
) -> Result<KyaObjectRef, Error> {
    if Arc::ptr_eq(&obj1, &obj2) {
        return Ok(bool_new(true));
    }

    if obj1.lock().unwrap().is_instance_of(&*NONE_TYPE)?
        && obj2.lock().unwrap().is_instance_of(&*NONE_TYPE)?
    {
        return Ok(bool_new(true));
    }

    Ok(bool_new(false))
}

pub static NONE_TYPE: Lazy<TypeRef> = Lazy::new(|| {
    Type::as_ref(Type {
        ob_type: Some(BASE_TYPE.clone()),
        name: "None".to_string(),
        tp_repr: Some(none_repr),
        tp_compare: Some(none_tp_compare),
        ..Default::default()
    })
});
