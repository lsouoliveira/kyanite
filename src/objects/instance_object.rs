use crate::errors::Error;
use crate::objects::base::{
    DictRef, KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef, BASE_TYPE,
};
// use crate::objects::method_object::MethodObject;
use crate::objects::string_object::{StringObject, STRING_TYPE};
use crate::objects::utils::parse_arg;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct InstanceObject {
    pub ob_type: TypeRef,
    pub dict: DictRef,
}

impl KyaObjectTrait for InstanceObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn instance_new(
    ob_type: TypeRef,
    args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let _arg = parse_arg(&args, 0, 1)?;

    Ok(KyaObject::from_instance_object(InstanceObject {
        ob_type,
        dict: Arc::new(Mutex::new(HashMap::new())),
    }))
}

pub fn instance_tp_init(
    callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.lock().unwrap();

    if let KyaObject::InstanceObject(_) = &*object {
        let constructor_option = object
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

        if let Some(init) = constructor_option {
            let init_type = init.lock().unwrap().get_type()?;
            let tp_call = init_type.lock().unwrap().tp_call;

            if tp_call.is_none() {
                return Err(Error::RuntimeError(format!(
                    "The object '{}' has no callable constructor",
                    object.get_type()?.lock().unwrap().name
                )));
            }

            let result = tp_call.unwrap()(init.clone(), args, receiver);

            result
        } else {
            if args.is_empty() {
                Ok(callable.clone())
            } else {
                Err(Error::RuntimeError(format!(
                    "The object '{}' takes no arguments, but {} were given",
                    object.get_type()?.lock().unwrap().name,
                    args.len()
                )))
            }
        }
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a instance",
            object.get_type()?.lock().unwrap().name
        )))
    }
}

pub fn instance_tp_repr(
    callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.lock().unwrap();

    if let KyaObject::InstanceObject(_) = &*object {
        let ob_type = callable.lock().unwrap().get_type()?;
        let tp_get_attr_fn = ob_type.lock().unwrap().tp_get_attr;

        if tp_get_attr_fn.is_none() {
            return instance_default_repr(callable.clone(), args, receiver);
        }

        let repr = tp_get_attr_fn.unwrap()(callable.clone(), "__repr__".to_string());

        if let Ok(repr) = repr {
            let result = repr.lock().unwrap().get_type()?.lock().unwrap().call(
                repr.clone(),
                &mut vec![],
                receiver,
            );

            result
        } else {
            instance_default_repr(callable.clone(), args, receiver)
        }
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a instance",
            object.get_type()?.lock().unwrap().name
        )))
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
    let object = obj.lock().unwrap();

    if let KyaObject::InstanceObject(instance_object) = &*object {
        let found_object = get_attr(obj.clone(), instance_object, attr_name.clone())?;

        // if let KyaObject::FunctionObject(_) = &*found_object.lock().unwrap() {
        //     return Ok(KyaObject::from_method_object(MethodObject {
        //         ob_type: interpreter.get_type(METHOD_TYPE),
        //         instance_object: obj.clone(),
        //         function: found_object.clone(),
        //     }));
        // } else if let KyaObject::RsFunctionObject(_) = &*found_object.lock().unwrap() {
        //     return Ok(KyaObject::from_method_object(MethodObject {
        //         ob_type: interpreter.get_type(METHOD_TYPE),
        //         instance_object: obj.clone(),
        //         function: found_object.clone(),
        //     }));
        // }

        return Ok(found_object);
    }

    Err(Error::RuntimeError(format!(
        "The object '{}' has no attribute '{}'",
        object.get_type()?.lock().unwrap().name,
        attr_name
    )))
}

pub fn get_attr(
    object: KyaObjectRef,
    instance_object: &InstanceObject,
    attr_name: String,
) -> Result<KyaObjectRef, Error> {
    if let Some(attr) = instance_object.dict.lock().unwrap().get(&attr_name) {
        return Ok(attr.clone());
    } else {
        let mut root_type = object.lock().unwrap().get_type()?;
        let mut parent_type = root_type.lock().unwrap().parent()?;

        loop {
            if Arc::ptr_eq(&parent_type, &BASE_TYPE) {
                break;
            }

            if let Some(attr) = root_type
                .lock()
                .unwrap()
                .dict
                .lock()
                .unwrap()
                .get(&attr_name)
            {
                return Ok(attr.clone());
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
