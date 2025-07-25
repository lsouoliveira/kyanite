use crate::errors::Error;
use crate::objects::base::{
    CallableFunctionPtr, KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef, BASE_TYPE,
};

use once_cell::sync::Lazy;

pub struct RsFunctionObject {
    pub ob_type: TypeRef,
    pub function_ptr: CallableFunctionPtr,
}

impl RsFunctionObject {
    pub fn new(ob_type: TypeRef, function_ptr: CallableFunctionPtr) -> Self {
        Self {
            ob_type,
            function_ptr,
        }
    }
}

impl KyaObjectTrait for RsFunctionObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn rs_function_tp_call(
    callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let function_pointer = if let KyaObject::RsFunctionObject(function) = &*callable.lock().unwrap()
    {
        Ok(function.function_ptr.clone())
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not callable",
            callable.lock().unwrap().get_type()?.lock().unwrap().name
        )))
    }?;

    (function_pointer)(callable.clone(), args, receiver)
}

pub fn rs_function_new(function_ptr: CallableFunctionPtr) -> KyaObjectRef {
    KyaObject::from_rs_function_object(RsFunctionObject::new(
        RS_FUNCTION_TYPE.clone(),
        function_ptr,
    ))
}

pub static RS_FUNCTION_TYPE: Lazy<TypeRef> = Lazy::new(|| {
    Type::as_ref(Type {
        ob_type: Some(BASE_TYPE.clone()),
        name: "RsFunction".to_string(),
        tp_call: Some(rs_function_tp_call),
        ..Default::default()
    })
});
