use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::{
    kya_string_as_string, Context, KyaInstanceObject, KyaObject, KyaRsFunction, KyaString,
};

use std::rc::Rc;
pub fn kya_string_repr(
    interpreter: &mut Interpreter,
    _: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let instance = interpreter.get_self()?;

    Ok(instance)
}

pub fn kya_string_length(
    interpreter: &mut Interpreter,
    _: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let instance = interpreter.resolve("self").ok_or_else(|| {
        Error::RuntimeError("String object does not have a self attribute".to_string())
    })?;

    if let KyaObject::InstanceObject(obj) = instance.as_ref() {
        if let Some(value) = obj.get_attribute("__value__") {
            if let KyaObject::String(value) = value.as_ref() {
                return Ok(Rc::new(KyaObject::Number(value.value.len() as f64)));
            }
        }
    }

    Err(Error::RuntimeError(
        "String object does not have a __value__ attribute".to_string(),
    ))
}

pub fn kya_string_new(value: &str) -> Result<Rc<KyaObject>, Error> {
    let mut locals = Context::new();

    locals.register(
        String::from("__value__"),
        Rc::new(KyaObject::String(KyaString::new(value.to_string()))),
    );

    locals.register(
        String::from("__repr__"),
        Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
            String::from("__repr__"),
            kya_string_repr,
        ))),
    );

    locals.register(
        String::from("length"),
        Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
            String::from("length"),
            kya_string_length,
        ))),
    );

    locals.register(
        String::from("__eq__"),
        Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
            String::from("__eq__"),
            kya_string_eq,
        ))),
    );

    locals.register(
        String::from("__add__"),
        Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
            String::from("__add__"),
            kya_string_add,
        ))),
    );

    Ok(Rc::new(KyaObject::InstanceObject(KyaInstanceObject::new(
        "String".to_string(),
        locals,
    ))))
}

fn kya_string_get_value(interpreter: &mut Interpreter) -> Result<String, Error> {
    let instance = interpreter.get_self()?;

    if let KyaObject::InstanceObject(obj) = instance.as_ref() {
        return Ok(obj.get_string_attribute("__value__").unwrap());
    }

    Err(Error::RuntimeError(
        "String object does not have a __value__ attribute".to_string(),
    ))
}

pub fn kya_string_eq(
    interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    if args.len() != 1 {
        return Err(Error::TypeError(
            "eq() requires exactly one argument".to_string(),
        ));
    }

    if let KyaObject::InstanceObject(obj) = args[0].as_ref() {
        if obj.name() != "String" {
            return Ok(interpreter.false_object());
        }

        let self_value = kya_string_get_value(interpreter)?;
        let other_value = obj.get_string_attribute("__value__").unwrap();

        if self_value == other_value {
            return Ok(interpreter.true_object());
        } else {
            return Ok(interpreter.false_object());
        }
    }

    Err(Error::RuntimeError(
        "String object does not have a __value__ attribute".to_string(),
    ))
}

pub fn kya_string_add(
    interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    if args.len() != 1 {
        return Err(Error::TypeError(
            "add() requires exactly one argument".to_string(),
        ));
    }

    let other_value = kya_string_as_string(&args[0])?;
    let self_value = kya_string_get_value(interpreter)?;

    Ok(kya_string_new(&(self_value + &other_value)).unwrap())
}
