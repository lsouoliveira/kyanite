use crate::errors::Error;
use crate::objects::base::{
    kya_sq_len, KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef, BASE_TYPE,
};
use crate::objects::number_object::number_new;
use crate::objects::rs_function_object::rs_function_new;
use crate::objects::string_object::string_new;
use crate::objects::utils::parse_receiver;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct BytesObject {
    pub ob_type: TypeRef,
    pub value: Vec<u8>,
}

impl KyaObjectTrait for BytesObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn bytes_new(value: Vec<u8>) -> KyaObjectRef {
    KyaObject::from_bytes_object(BytesObject {
        ob_type: BYTES_TYPE.clone(),
        value,
    })
}

pub fn bytes_tp_repr(
    callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.lock().unwrap();

    if let KyaObject::BytesObject(obj) = &*object {
        Ok(string_new(
            format!("b'{}'", String::from_utf8_lossy(&obj.value)).as_str(),
        ))
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a bytes object.",
            object.get_type()?.lock().unwrap().name
        )))
    }
}

pub fn bytes_sq_len(object: KyaObjectRef) -> Result<usize, Error> {
    if let KyaObject::BytesObject(obj) = &*object.lock().unwrap() {
        Ok(obj.value.len())
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a bytes object.",
            object.lock().unwrap().get_type()?.lock().unwrap().name
        )))
    }
}

pub fn bytes_length(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let instance = parse_receiver(&receiver)?;

    if !matches!(&*instance.lock().unwrap(), KyaObject::BytesObject(_)) {
        return Err(Error::RuntimeError(format!(
            "The object '{}' is not a bytes object.",
            instance.lock().unwrap().get_type()?.lock().unwrap().name
        )));
    }

    let bytes_length = kya_sq_len(instance.clone())?;

    Ok(number_new(bytes_length as f64))
}

pub fn bytes_decode(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let instance = parse_receiver(&receiver)?;

    if let KyaObject::BytesObject(obj) = &*instance.lock().unwrap() {
        let decoded_string = String::from_utf8_lossy(&obj.value).to_string();
        Ok(string_new(decoded_string.as_str()))
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a bytes object.",
            instance.lock().unwrap().get_type()?.lock().unwrap().name
        )))
    }
}

pub static BYTES_TYPE: Lazy<TypeRef> = Lazy::new(|| {
    let dict = Arc::new(Mutex::new(HashMap::new()));

    dict.lock()
        .unwrap()
        .insert("length".to_string(), rs_function_new(bytes_length));

    dict.lock()
        .unwrap()
        .insert("decode".to_string(), rs_function_new(bytes_decode));

    Type::as_ref(Type {
        ob_type: Some(BASE_TYPE.clone()),
        name: "Bytes".to_string(),
        tp_repr: Some(bytes_tp_repr),
        sq_len: Some(bytes_sq_len),
        dict: dict.clone(),
        ..Default::default()
    })
});
