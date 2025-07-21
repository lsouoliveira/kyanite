use crate::errors::Error;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef, BASE_TYPE};
use crate::objects::string_object::{string_new, StringObject, STRING_TYPE};

use once_cell::sync::Lazy;

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

pub static NONE_TYPE: Lazy<TypeRef> = Lazy::new(|| {
    Type::as_ref(Type {
        ob_type: Some(BASE_TYPE.clone()),
        name: "None".to_string(),
        tp_repr: Some(none_repr),
        ..Default::default()
    })
});
