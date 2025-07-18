use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::base::{DictRef, KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef};
use crate::objects::instance_object::{create_instance_type, instance_tp_repr, InstanceObject};
use crate::objects::string_object::StringObject;

pub struct ClassObject {
    pub ob_type: TypeRef,
}

impl KyaObjectTrait for ClassObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn create_class_type(interpreter: &mut Interpreter, name: String, dict: DictRef) -> TypeRef {
    Type::as_ref(Type {
        ob_type: Some(interpreter.get_type("Type")),
        name: name.clone(),
        tp_call: Some(class_tp_call),
        tp_new: Some(class_tp_new),
        tp_repr: Some(class_tp_repr),
        dict: dict.clone(),
        ..Default::default()
    })
}

pub fn class_tp_call(
    interpreter: &mut Interpreter,
    callable: KyaObjectRef,
    args: Vec<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let class_type = callable.borrow().get_type()?;

    let obj = class_type
        .borrow()
        .new(interpreter, class_type.clone(), args.clone())?;

    obj.borrow()
        .get_type()?
        .borrow()
        .init(interpreter, obj.clone(), args)?;

    Ok(obj)
}

pub fn class_tp_new(
    _interpreter: &mut Interpreter,
    ob_type: TypeRef,
    _args: Vec<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let instance_ob_type = create_instance_type(ob_type);

    Ok(KyaObject::from_instance_object(InstanceObject {
        ob_type: instance_ob_type,
        dict: Rc::new(RefCell::new(HashMap::new())),
    }))
}

pub fn class_tp_repr(
    _interpreter: &mut Interpreter,
    callable: KyaObjectRef,
    _args: Vec<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.borrow();

    if let KyaObject::ClassObject(_) = &*object {
        Ok(KyaObject::from_string_object(StringObject {
            ob_type: _interpreter.get_type("String"),
            value: format!(
                "<class {} at {:p}>
            ",
                object.get_type()?.borrow().name,
                &*object as *const KyaObject
            ),
        }))
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a class",
            object.get_type()?.borrow().name
        )))
    }
}
