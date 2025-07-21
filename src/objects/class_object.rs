use std::collections::HashMap;

use crate::errors::Error;
// use crate::interpreter::Interpreter;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, TypeRef};
// use crate::objects::instance_object::{create_instance_type, InstanceObject};
// use crate::objects::string_object::StringObject;

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

    let obj = class_type.lock().unwrap().new(
        class_type.clone(),
        &mut args.clone(),
        Some(callable.clone()),
    )?;

    obj.lock()
        .unwrap()
        .get_type()?
        .lock()
        .unwrap()
        .init(obj.clone(), args, Some(obj.clone()))?;

    Ok(obj)
}

pub fn class_tp_new(
    _ob_type: TypeRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    // let instance_ob_type = create_instance_type(ob_type);
    //
    // Ok(KyaObject::from_instance_object(InstanceObject {
    //     ob_type: instance_ob_type,
    //     dict: Rc::new(RefCell::new(HashMap::new())),
    // }))

    Err(Error::RuntimeError(
        "Class tp_new is not implemented".to_string(),
    ))
}

pub fn class_tp_repr(
    callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.lock().unwrap();

    if let KyaObject::ClassObject(_) = &*object {
        // Ok(KyaObject::from_string_object(StringObject {
        //     ob_type: _interpreter.get_type("String"),
        //     value: format!(
        //         "<class {} at {:p}>
        //     ",
        //         object.get_type()?.lock().unwrap().name,
        //         &*object as *const KyaObject
        //     ),
        // }))

        Err(Error::RuntimeError(format!(
            "The object '{}' is not a class",
            object.get_type()?.lock().unwrap().name
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
