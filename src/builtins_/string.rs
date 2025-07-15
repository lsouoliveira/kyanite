use crate::builtins_::list::kya_list_new;
use crate::builtins_::number::kya_number_new;
use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::{
    kya_string_as_string, unpack_string, Context, KyaInstanceObject, KyaMethod, KyaNone, KyaObject,
    KyaRsFunction, KyaString,
};

use std::cell::RefCell;
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

pub fn kya_string_init(
    interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let instance = interpreter.get_self()?;
    let arg = unpack_string(&args, 0, 1)
        .unwrap_or_else(|_| Rc::new(KyaObject::String(KyaString::new("".to_string()))));
    let value = kya_string_as_string(&arg)?;

    if let KyaObject::InstanceObject(obj) = instance.as_ref() {
        obj.set_attribute(
            "__value__".to_string(),
            Rc::new(KyaObject::String(KyaString::new(value))),
        );
    }

    Ok(Rc::new(KyaObject::None(KyaNone {})))
}

pub fn kya_string_new(value: &str) -> Result<Rc<KyaObject>, Error> {
    let mut locals = Context::new();

    locals.register(
        String::from("__value__"),
        Rc::new(KyaObject::String(KyaString::new(value.to_string()))),
    );

    let instance = Rc::new(KyaObject::InstanceObject(KyaInstanceObject::new(
        "String".to_string(),
        RefCell::new(locals),
    )));

    if let KyaObject::InstanceObject(instance_obj) = instance.as_ref() {
        instance_obj.set_attribute(
            String::from("constructor"),
            Rc::new(KyaObject::Method(KyaMethod {
                function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                    String::from("constructor"),
                    kya_string_init,
                ))),
                instance: instance.clone(),
            })),
        );

        instance_obj.set_attribute(
            String::from("__repr__"),
            Rc::new(KyaObject::Method(KyaMethod {
                function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                    String::from("__repr__"),
                    kya_string_repr,
                ))),
                instance: instance.clone(),
            })),
        );

        instance_obj.set_attribute(
            String::from("length"),
            Rc::new(KyaObject::Method(KyaMethod {
                function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                    String::from("length"),
                    kya_string_length,
                ))),
                instance: instance.clone(),
            })),
        );

        instance_obj.set_attribute(
            String::from("__eq__"),
            Rc::new(KyaObject::Method(KyaMethod {
                function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                    String::from("__eq__"),
                    kya_string_eq,
                ))),
                instance: instance.clone(),
            })),
        );

        instance_obj.set_attribute(
            String::from("__add__"),
            Rc::new(KyaObject::Method(KyaMethod {
                function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                    String::from("__add__"),
                    kya_string_add,
                ))),
                instance: instance.clone(),
            })),
        );

        instance_obj.set_attribute(
            String::from("split"),
            Rc::new(KyaObject::Method(KyaMethod {
                function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                    String::from("split"),
                    kya_string_split,
                ))),
                instance: instance.clone(),
            })),
        );

        instance_obj.set_attribute(
            String::from("to_i"),
            Rc::new(KyaObject::Method(KyaMethod {
                function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                    String::from("to_i"),
                    kya_string_to_i,
                ))),
                instance: instance.clone(),
            })),
        );
    }

    Ok(instance)
}

pub fn instantiate_string(
    _: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let arg = unpack_string(&args, 0, 1)
        .unwrap_or_else(|_| Rc::new(KyaObject::String(KyaString::new("".to_string()))));
    let value = kya_string_as_string(&arg)?;

    kya_string_new(&value)
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

pub fn kya_string_split(
    interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let separator = unpack_string(&args, 0, 1)?;
    let separator_value = kya_string_as_string(&separator)?;
    let instance = interpreter.get_self()?;
    let value = kya_string_as_string(&instance)?;
    let items = value
        .split(&separator_value)
        .map(|s| kya_string_new(s).unwrap())
        .collect::<Vec<Rc<KyaObject>>>();

    Ok(kya_list_new(items).unwrap())
}

pub fn kya_string_to_i(
    interpreter: &mut Interpreter,
    _: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let instance = interpreter.get_self()?;
    let value = kya_string_as_string(&instance)?;

    match value.parse::<i64>() {
        Ok(num) => Ok(kya_number_new(num as f64).unwrap()),
        Err(_) => Err(Error::RuntimeError(
            "Cannot convert string to integer".to_string(),
        )),
    }
}
