use crate::errors::Error;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, TypeRef};
use crate::objects::instance_object::{instance_type_new, InstanceObject};
use crate::objects::string_object::string_new;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ClassObject {
    pub ob_type: TypeRef,
}

impl KyaObjectTrait for ClassObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn class_tp_call(
    callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let class_type = callable.lock().unwrap().get_type()?;

    let tp_new = class_type.lock().unwrap().tp_new;

    if let Some(new_fn) = tp_new {
        let obj = new_fn(
            class_type.clone(),
            &mut args.clone(),
            Some(callable.clone()),
        )?;

        let tp_init = class_type.lock().unwrap().tp_init;

        if let Some(init_fn) = tp_init {
            init_fn(obj.clone(), args, Some(obj.clone()))?;
        } else {
            return Err(Error::RuntimeError(format!(
                "Class '{}' does not have a tp_init method",
                class_type.lock().unwrap().name
            )));
        }

        Ok(obj)
    } else {
        return Err(Error::RuntimeError(format!(
            "Class '{}' does not have a tp_new method",
            class_type.lock().unwrap().name
        )));
    }
}

pub fn class_tp_new(
    ob_type: TypeRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    Ok(KyaObject::from_instance_object(InstanceObject {
        ob_type: instance_type_new(ob_type),
        dict: Arc::new(Mutex::new(HashMap::new())),
    }))
}

pub fn class_tp_repr(
    callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.lock().unwrap();

    if let KyaObject::ClassObject(_) = &*object {
        Ok(string_new(&format!(
            "<class {} at {:p}>",
            object.get_type()?.lock().unwrap().name.as_str(),
            &*object as *const KyaObject
        )))
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a class",
            object.get_type()?.lock().unwrap().name
        )))
    }
}

pub fn class_nb_bool(_: KyaObjectRef) -> Result<f64, Error> {
    Ok(1.0)
}

pub fn class_new(ob_type: TypeRef) -> KyaObjectRef {
    KyaObject::from_class_object(ClassObject { ob_type })
}
