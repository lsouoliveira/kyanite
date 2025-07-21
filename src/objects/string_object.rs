use crate::errors::Error;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef, BASE_TYPE};
use once_cell::sync::Lazy;

pub struct StringObject {
    pub ob_type: TypeRef,
    pub value: String,
}

impl KyaObjectTrait for StringObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn string_new(value: &str) -> KyaObjectRef {
    KyaObject::from_string_object(StringObject {
        ob_type: STRING_TYPE.clone(),
        value: value.to_string(),
    })
}

pub fn string_tp_repr(
    callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.lock().unwrap();

    if let KyaObject::StringObject(_) = &*object {
        Ok(callable.clone())
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a string",
            object.get_type()?.lock().unwrap().name
        )))
    }
}

pub static STRING_TYPE: Lazy<TypeRef> = Lazy::new(|| {
    Type::as_ref(Type {
        ob_type: Some(BASE_TYPE.clone()),
        name: "String".to_string(),
        tp_repr: Some(string_tp_repr),
        ..Default::default()
    })
});
