use crate::errors::Error;
use crate::interpreter::{Interpreter, METHOD_TYPE};
use crate::objects::base::{DictRef, KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef};
use crate::objects::method_object::MethodObject;
use crate::objects::string_object::StringObject;
use crate::objects::utils::parse_arg;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct InstanceObject {
    pub ob_type: TypeRef,
    pub dict: DictRef,
}

impl KyaObjectTrait for InstanceObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn create_instance_type(ob_type: TypeRef) -> TypeRef {
    Type::as_ref(Type {
        ob_type: Some(ob_type.clone()),
        name: ob_type.lock().unwrap().name.clone(),
        tp_init: Some(instance_tp_init),
        tp_repr: Some(instance_tp_repr),
        tp_get_attr: Some(instance_tp_get_attr),
        tp_set_attr: Some(instance_tp_set_attr),
        ..Default::default()
    })
}

pub fn instance_new(
    _interpreter: &mut Interpreter,
    ob_type: TypeRef,
    args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let _arg = parse_arg(&args, 0, 1)?;

    Ok(KyaObject::from_instance_object(InstanceObject {
        ob_type,
        dict: Rc::new(RefCell::new(HashMap::new())),
    }))
}

pub fn instance_tp_init(
    interpreter: &mut Interpreter,
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
            let result = init.lock().unwrap().get_type()?.lock().unwrap().call(
                interpreter,
                init.clone(),
                args,
                receiver,
            );

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
    interpreter: &mut Interpreter,
    callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.lock().unwrap();

    if let KyaObject::InstanceObject(_) = &*object {
        let ob_type = callable.lock().unwrap().get_type()?;
        let repr =
            ob_type
                .lock()
                .unwrap()
                .get_attr(interpreter, callable.clone(), "__repr__".to_string());

        if let Ok(repr) = repr {
            let result = repr.lock().unwrap().get_type()?.lock().unwrap().call(
                interpreter,
                repr.clone(),
                &mut vec![],
                receiver,
            );

            result
        } else {
            instance_default_repr(interpreter, callable.clone(), args, receiver)
        }
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a instance",
            object.get_type()?.lock().unwrap().name
        )))
    }
}

pub fn instance_default_repr(
    _interpreter: &mut Interpreter,
    callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.lock().unwrap();

    if let KyaObject::InstanceObject(_) = &*object {
        Ok(KyaObject::from_string_object(StringObject {
            ob_type: _interpreter.get_type("String"),
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

pub fn instance_tp_get_attr(
    interpreter: &mut Interpreter,
    obj: KyaObjectRef,
    attr_name: String,
) -> Result<KyaObjectRef, Error> {
    let object = obj.lock().unwrap();

    if let KyaObject::InstanceObject(instance_object) = &*object {
        let found_object = get_attr(obj.clone(), instance_object, attr_name.clone())?;

        if let KyaObject::FunctionObject(_) = &*found_object.lock().unwrap() {
            return Ok(KyaObject::from_method_object(MethodObject {
                ob_type: interpreter.get_type(METHOD_TYPE),
                instance_object: obj.clone(),
                function: found_object.clone(),
            }));
        } else if let KyaObject::RsFunctionObject(_) = &*found_object.lock().unwrap() {
            return Ok(KyaObject::from_method_object(MethodObject {
                ob_type: interpreter.get_type(METHOD_TYPE),
                instance_object: obj.clone(),
                function: found_object.clone(),
            }));
        }

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
            if Rc::ptr_eq(&root_type, &parent_type) {
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
    _interpreter: &mut Interpreter,
    obj: KyaObjectRef,
    attr_name: String,
    value: KyaObjectRef,
) -> Result<(), Error> {
    let object = obj.lock().unwrap();

    if let KyaObject::InstanceObject(obj) = &*object {
        obj.dict.lock_mut().insert(attr_name, value);
        Ok(())
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a instance",
            object.get_type()?.lock().unwrap().name
        )))
    }
}
