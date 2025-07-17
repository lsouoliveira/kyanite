use crate::builtins_::string::kya_string_new;
use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::{
    unpack_args, Context, KyaBool, KyaInstanceObject, KyaMethod, KyaObject, KyaRsFunction,
};

use std::cell::RefCell;
use std::rc::Rc;

pub fn kya_bool_eq(
    interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    unpack_args(&args, 0, 1)?;
    let instance = interpreter.get_self()?;

    let self_value = instance.as_bool()?;
    let other_value = args[0].as_bool()?;

    if self_value == other_value {
        return Ok(interpreter.true_object());
    } else {
        return Ok(interpreter.false_object());
    }
}

pub fn kya_bool_new(value: bool) -> Result<Rc<KyaObject>, Error> {
    let instance = Rc::new(KyaObject::InstanceObject(KyaInstanceObject::new(
        "Bool".to_string(),
        RefCell::new(Context::new()),
    )));

    let object = Rc::new(KyaObject::Bool(KyaBool {
        value: RefCell::new(value),
        instance: instance.clone(),
    }));

    instance.set_attribute(
        String::from("__repr__"),
        Rc::new(KyaObject::Method(KyaMethod {
            function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                String::from("__repr__"),
                kya_bool_repr,
            ))),
            instance: object.clone(),
        })),
    );

    instance.set_attribute(
        String::from("__eq__"),
        Rc::new(KyaObject::Method(KyaMethod {
            function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                String::from("__eq__"),
                kya_bool_eq,
            ))),
            instance: object.clone(),
        })),
    );

    Ok(object)
}

pub fn kya_bool_repr(
    interpreter: &mut Interpreter,
    _: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let instance = interpreter.get_self()?;

    Ok(kya_string_new(&instance.as_bool()?.to_string())?)
}
