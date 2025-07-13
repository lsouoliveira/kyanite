use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::{Context, KyaInstanceObject, KyaNone, KyaObject, KyaRsFunction, KyaString};
use std::rc::Rc;

pub fn kya_print(
    interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    if args.is_empty() {
        return Err(Error::TypeError(
            "print() requires at least one argument".to_string(),
        ));
    }

    let mut output = String::new();

    for arg in args {
        if let KyaObject::InstanceObject(_) = arg.as_ref() {
            let result = interpreter.call_instance_method(arg.clone(), "__repr__", vec![])?;

            if let KyaObject::String(result) = result.as_ref() {
                output.push_str(&result.value);
            } else {
                return Err(Error::TypeError(
                    "__repr__ must return a string".to_string(),
                ));
            }
        } else {
            output.push_str(&arg.repr());
        }
    }

    println!("{}", output);

    Ok(Rc::new(KyaObject::None(KyaNone {})))
}

pub fn kya_globals(
    interpreter: &mut Interpreter,
    _: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    for name in interpreter.context.keys() {
        println!("{}", name);
    }

    Ok(Rc::new(KyaObject::None(KyaNone {})))
}

pub fn kya_string_repr(
    interpreter: &mut Interpreter,
    _: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let instance = interpreter.resolve("self").ok_or_else(|| {
        Error::RuntimeError("String object does not have a self attribute".to_string())
    })?;

    if let KyaObject::InstanceObject(obj) = instance.as_ref() {
        if let Some(value) = obj.get_attribute("__value__") {
            if let KyaObject::String(value) = value.as_ref() {
                return Ok(Rc::new(KyaObject::String(KyaString {
                    value: value.value.to_string(),
                })));
            } else if let KyaObject::Number(value) = value.as_ref() {
                return Ok(Rc::new(KyaObject::String(KyaString {
                    value: value.to_string(),
                })));
            }
        }
    }

    Err(Error::RuntimeError(
        "String object does not have a __value__ attribute".to_string(),
    ))
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

    Ok(Rc::new(KyaObject::InstanceObject(KyaInstanceObject::new(
        locals,
    ))))
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

    Ok(Rc::new(KyaObject::InstanceObject(KyaInstanceObject::new(
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
