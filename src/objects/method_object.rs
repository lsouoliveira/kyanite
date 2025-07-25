use crate::errors::Error;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef, BASE_TYPE};
use crate::objects::string_object::string_new;
use once_cell::sync::Lazy;

pub struct MethodObject {
    pub ob_type: TypeRef,
    pub function: KyaObjectRef,
    pub instance_object: KyaObjectRef,
}

impl KyaObjectTrait for MethodObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn method_tp_repr(
    callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.lock().unwrap();

    if let KyaObject::MethodObject(method_object) = &*object {
        let instance_type = method_object.instance_object.lock().unwrap().get_type()?;

        Ok(string_new(&format!(
            "<{} method at {:p} for {}>",
            object.get_type()?.lock().unwrap().name,
            &*object as *const KyaObject,
            instance_type.lock().unwrap().name
        )))
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a method",
            object.get_type()?.lock().unwrap().name
        )))
    }
}

pub fn method_tp_call(
    callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let function_object;
    let instance_object;

    if let KyaObject::MethodObject(method_object) = &*callable.lock().unwrap() {
        function_object = method_object.function.clone();
        instance_object = method_object.instance_object.clone();
    } else {
        return Err(Error::RuntimeError(format!(
            "The object '{}' is not a method",
            callable.lock().unwrap().get_type()?.lock().unwrap().name
        )));
    };

    let function_type = function_object.lock().unwrap().get_type()?;
    let tp_call = function_type.lock().unwrap().tp_call.clone();

    if tp_call.is_none() {
        return Err(Error::RuntimeError(format!("The method is not callable",)));
    }

    let call_fn = tp_call.unwrap();

    call_fn(function_object.clone(), args, Some(instance_object.clone()))
}

pub static METHOD_TYPE: Lazy<TypeRef> = Lazy::new(|| {
    Type::as_ref(Type {
        ob_type: Some(BASE_TYPE.clone()),
        name: "Method".to_string(),
        tp_repr: Some(method_tp_repr),
        tp_call: Some(method_tp_call),
        ..Default::default()
    })
});
