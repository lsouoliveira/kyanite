use crate::builtins_::string::kya_string_new;
use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::{
    kya_number_as_float, Context, KyaInstanceObject, KyaObject, KyaRsFunction, KyaString,
};
use std::cell::RefCell;
use std::rc::Rc;

pub fn kya_number_repr(
    interpreter: &mut Interpreter,
    _: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let instance = interpreter.resolve("self").ok_or_else(|| {
        Error::RuntimeError("Number object does not have a self attribute".to_string())
    })?;

    if let KyaObject::InstanceObject(obj) = instance.as_ref() {
        if let Some(value) = obj.get_attribute("__value__") {
            if let KyaObject::Number(value) = value.as_ref() {
                return kya_string_new(&value.to_string());
            }
        }
    }

    Err(Error::RuntimeError(
        "Number object does not have a __value__ attribute".to_string(),
    ))
}

pub fn kya_number_get_value(interpreter: &mut Interpreter) -> Result<f64, Error> {
    let instance = interpreter.get_self()?;

    if let KyaObject::InstanceObject(obj) = instance.as_ref() {
        return Ok(obj.get_number_attribute("__value__").unwrap());
    }

    Err(Error::RuntimeError(
        "Number object does not have a __value__ attribute".to_string(),
    ))
}

pub fn kya_number_eq(
    interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    if args.len() != 1 {
        return Err(Error::TypeError(
            "eq() requires exactly one argument".to_string(),
        ));
    }

    if let KyaObject::InstanceObject(obj) = args[0].as_ref() {
        if obj.name() != "Number" {
            return Ok(interpreter.false_object());
        }

        let self_value = kya_number_get_value(interpreter)?;
        let other_value = obj.get_number_attribute("__value__").unwrap();

        if self_value == other_value {
            return Ok(interpreter.true_object());
        } else {
            return Ok(interpreter.false_object());
        }
    }

    Err(Error::RuntimeError(
        "Number object does not have a __value__ attribute".to_string(),
    ))
}

pub fn kya_number_add(
    interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let other_value = kya_number_as_float(&args[0])?;
    let self_value = kya_number_as_float(&interpreter.get_self().unwrap())?;

    Ok(kya_number_new(self_value + other_value).unwrap())
}

pub fn kya_number_new(value: f64) -> Result<Rc<KyaObject>, Error> {
    let mut locals = Context::new();
    let obj = KyaObject::Number(value);

    locals.register(String::from("__value__"), Rc::new(obj.clone()));

    locals.register(
        String::from("__repr__"),
        Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
            String::from("__repr__"),
            kya_number_repr,
        ))),
    );

    locals.register(
        String::from("__eq__"),
        Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
            String::from("__eq__"),
            kya_number_eq,
        ))),
    );

    locals.register(
        String::from("__add__"),
        Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
            String::from("__add__"),
            kya_number_add,
        ))),
    );

    Ok(Rc::new(KyaObject::InstanceObject(KyaInstanceObject::new(
        "Number".to_string(),
        RefCell::new(locals),
    ))))
}
