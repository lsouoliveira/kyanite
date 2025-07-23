use crate::bytecode;
use crate::errors::Error;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef, BASE_TYPE};
use crate::objects::string_object::string_new;

use once_cell::sync::Lazy;
use std::sync::Arc;

pub struct CodeObject {
    ob_type: TypeRef,
    pub code: Arc<bytecode::CodeObject>,
}

impl KyaObjectTrait for CodeObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn code_object_new(code: Arc<bytecode::CodeObject>) -> KyaObjectRef {
    KyaObject::from_code_object(CodeObject {
        ob_type: CODE_TYPE.clone(),
        code,
    })
}

pub fn none_repr(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    Ok(string_new(&format!("<Code object at {:p}>", _callable)))
}

pub static CODE_TYPE: Lazy<TypeRef> = Lazy::new(|| {
    Type::as_ref(Type {
        ob_type: Some(BASE_TYPE.clone()),
        name: "Code".to_string(),
        tp_repr: Some(none_repr),
        ..Default::default()
    })
});
