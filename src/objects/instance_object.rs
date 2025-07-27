use crate::errors::Error;
use crate::objects::base::{
    kya_call, kya_get_attr, DictRef, KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef,
    BASE_TYPE,
};
use crate::objects::method_object::{MethodObject, METHOD_TYPE};
use crate::objects::string_object::{StringObject, STRING_TYPE};
use std::sync::Arc;

pub struct InstanceObject {
    pub ob_type: TypeRef,
    pub dict: DictRef,
}

impl KyaObjectTrait for InstanceObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn instance_tp_init(
    callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    if !matches!(&*callable.lock().unwrap(), KyaObject::InstanceObject(_)) {
        return Err(Error::RuntimeError(format!(
            "The object '{}' is not a instance",
            callable.lock().unwrap().get_type()?.lock().unwrap().name
        )));
    }

    let constructor = callable
        .lock()
        .unwrap()
        .get_type()?
        .lock()
        .unwrap()
        .parent()?
        .lock()
        .unwrap()
        .dict
        .lock()
        .unwrap()
        .get("constructor")
        .cloned();

    if let Some(init) = constructor {
        let result = kya_call(init, args, receiver);

        result
    } else {
        if args.is_empty() {
            Ok(callable.clone())
        } else {
            Err(Error::RuntimeError(format!(
                "The object '{}' takes no arguments, but {} were given",
                callable.lock().unwrap().get_type()?.lock().unwrap().name,
                args.len()
            )))
        }
    }
}

pub fn instance_tp_repr(
    callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let repr = kya_get_attr(callable.clone(), "__repr__".to_string());

    if repr.is_ok() {
        kya_call(repr.unwrap(), args, Some(callable.clone()))
    } else {
        instance_default_repr(callable, args, None)
    }
}

pub fn instance_default_repr(
    callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.lock().unwrap();

    if let KyaObject::InstanceObject(_) = &*object {
        Ok(KyaObject::from_string_object(StringObject {
            ob_type: STRING_TYPE.clone(),
            value: format!(
                "<instance {} at {:p}>",
                object.get_type()?.lock().unwrap().name,
                &*object as *const KyaObject
            ),
        }))
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a instance",
            object.get_type()?.lock().unwrap().name
        )))
    }
}

pub fn instance_tp_get_attr(obj: KyaObjectRef, attr_name: String) -> Result<KyaObjectRef, Error> {
    let dict_ref;

    if let KyaObject::InstanceObject(obj_instance) = &*obj.lock().unwrap() {
        dict_ref = obj_instance.dict.clone();
    } else {
        return Err(Error::RuntimeError(format!(
            "The object '{}' is not a instance",
            obj.lock().unwrap().get_type()?.lock().unwrap().name
        )));
    }

    let found_object = get_attr(obj.clone(), dict_ref, attr_name.clone())?;

    if let KyaObject::FunctionObject(_) = &*found_object.lock().unwrap() {
        return Ok(KyaObject::from_method_object(MethodObject {
            ob_type: METHOD_TYPE.clone(),
            instance_object: obj.clone(),
            function: found_object.clone(),
        }));
    } else if let KyaObject::RsFunctionObject(_) = &*found_object.lock().unwrap() {
        return Ok(KyaObject::from_method_object(MethodObject {
            ob_type: METHOD_TYPE.clone(),
            instance_object: obj.clone(),
            function: found_object.clone(),
        }));
    }

    Ok(found_object)
}

pub fn get_attr(
    object: KyaObjectRef,
    dict: DictRef,
    attr_name: String,
) -> Result<KyaObjectRef, Error> {
    if let Some(attr) = dict.lock().unwrap().get(&attr_name) {
        return Ok(attr.clone());
    } else {
        let mut root_type = object.lock().unwrap().get_type()?;
        let mut parent_type = root_type.lock().unwrap().parent()?;

        loop {
            if let Some(attr) = root_type
                .lock()
                .unwrap()
                .dict
                .lock()
                .unwrap()
                .get(&attr_name)
            {
                return Ok(attr.clone());
            } else if Arc::ptr_eq(&root_type, &BASE_TYPE) {
                break;
            }

            root_type = parent_type.clone();

            let new_parent_type = root_type.lock().unwrap().parent()?;

            parent_type = new_parent_type;
        }
    }

    Err(Error::RuntimeError(format!(
        "The object '{}' has no attribute '{}'",
        object.lock().unwrap().get_type()?.lock().unwrap().name,
        attr_name
    )))
}

pub fn instance_tp_set_attr(
    obj: KyaObjectRef,
    attr_name: String,
    value: KyaObjectRef,
) -> Result<(), Error> {
    let object = obj.lock().unwrap();

    if let KyaObject::InstanceObject(obj) = &*object {
        obj.dict.lock().unwrap().insert(attr_name, value);
        Ok(())
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a instance",
            object.get_type()?.lock().unwrap().name
        )))
    }
}

pub fn instance_type_new(ob_type: TypeRef) -> TypeRef {
    Type::as_ref(Type {
        ob_type: Some(ob_type.clone()),
        name: "Instance".to_string(),
        tp_init: Some(instance_tp_init),
        tp_repr: Some(instance_tp_repr),
        tp_get_attr: Some(instance_tp_get_attr),
        tp_set_attr: Some(instance_tp_set_attr),
        ..Default::default()
    })
}
