use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::{Context, KyaInstanceObject, KyaObject, KyaRsFunction, KyaString};
use std::rc::Rc;

pub fn kya_bool_get_value(interpreter: &mut Interpreter) -> Result<bool, Error> {
    let instance = interpreter.get_self()?;

    if let KyaObject::InstanceObject(obj) = instance.as_ref() {
        return Ok(obj.get_bool_attribute("__value__").unwrap());
    }

    Err(Error::RuntimeError(
        "Bool object does not have a __value__ attribute".to_string(),
    ))
}

pub fn kya_bool_eq(
    interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    if args.len() != 1 {
        return Err(Error::TypeError(
            "eq() requires exactly one argument".to_string(),
        ));
    }

    if let KyaObject::InstanceObject(obj) = args[0].as_ref() {
        if obj.name() != "Bool" {
            return Ok(interpreter.false_object());
        }

        let self_value = kya_bool_get_value(interpreter)?;
        let other_value = obj.get_bool_attribute("__value__").unwrap();

        if self_value == other_value {
            return Ok(interpreter.true_object());
        } else {
            return Ok(interpreter.false_object());
        }
    }

    Err(Error::RuntimeError(
        "Bool object does not have a __value__ attribute".to_string(),
    ))
}

pub fn kya_bool_new(value: bool) -> Result<Rc<KyaObject>, Error> {
    let mut locals = Context::new();
    let obj = KyaObject::Bool(value);

    locals.register(String::from("__value__"), Rc::new(obj.clone()));

    locals.register(
        String::from("__repr__"),
        Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
            String::from("__repr__"),
            kya_bool_repr,
        ))),
    );

    locals.register(
        String::from("__eq__"),
        Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
            String::from("__eq__"),
            kya_bool_eq,
        ))),
    );

    Ok(Rc::new(KyaObject::InstanceObject(KyaInstanceObject::new(
        "Bool".to_string(),
        locals,
    ))))
}

pub fn kya_bool_repr(
    interpreter: &mut Interpreter,
    _: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let instance = interpreter.resolve("self").ok_or_else(|| {
        Error::RuntimeError("Bool object does not have a self attribute".to_string())
    })?;

    if let KyaObject::InstanceObject(obj) = instance.as_ref() {
        if let Some(value) = obj.get_attribute("__value__") {
            if let KyaObject::Bool(value) = value.as_ref() {
                return Ok(Rc::new(KyaObject::String(KyaString::new(
                    value.to_string(),
                ))));
            }
        }
    }

    Err(Error::RuntimeError(
        "Bool object does not have a __value__ attribute".to_string(),
    ))
}
