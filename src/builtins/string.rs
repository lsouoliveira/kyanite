use crate::builtins_::list::kya_list_new;
use crate::builtins_::number::kya_number_new;
use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::{
    unpack_args, unpack_string, Context, KyaInstanceObject, KyaMethod, KyaNone, KyaObject,
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
    let instance = interpreter.get_self()?;
    let value = instance.as_string()?;

    Ok(kya_number_new(value.len() as f64).unwrap())
}

pub fn kya_string_init(
    interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let instance = interpreter.get_self()?;
    let arg = unpack_string(&args, 0, 1).unwrap_or_else(|_| kya_string_new("").unwrap());

    let _ = instance.assign(arg);

    Ok(Rc::new(KyaObject::None(KyaNone {})))
}

pub fn kya_string_new(value: &str) -> Result<Rc<KyaObject>, Error> {
    let instance = Rc::new(KyaObject::InstanceObject(KyaInstanceObject::new(
        "String".to_string(),
        RefCell::new(Context::new()),
    )));

    let object = Rc::new(KyaObject::String(KyaString {
        value: RefCell::new(value.to_string()),
        instance: instance.clone(),
    }));

    instance.set_attribute(
        String::from("constructor"),
        Rc::new(KyaObject::Method(KyaMethod {
            function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                String::from("constructor"),
                kya_string_init,
            ))),
            instance: object.clone(),
        })),
    );

    instance.set_attribute(
        String::from("__repr__"),
        Rc::new(KyaObject::Method(KyaMethod {
            function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                String::from("__repr__"),
                kya_string_repr,
            ))),
            instance: object.clone(),
        })),
    );

    instance.set_attribute(
        String::from("length"),
        Rc::new(KyaObject::Method(KyaMethod {
            function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                String::from("length"),
                kya_string_length,
            ))),
            instance: object.clone(),
        })),
    );

    instance.set_attribute(
        String::from("__eq__"),
        Rc::new(KyaObject::Method(KyaMethod {
            function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                String::from("__eq__"),
                kya_string_eq,
            ))),
            instance: object.clone(),
        })),
    );

    instance.set_attribute(
        String::from("__add__"),
        Rc::new(KyaObject::Method(KyaMethod {
            function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                String::from("__add__"),
                kya_string_add,
            ))),
            instance: object.clone(),
        })),
    );

    instance.set_attribute(
        String::from("split"),
        Rc::new(KyaObject::Method(KyaMethod {
            function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                String::from("split"),
                kya_string_split,
            ))),
            instance: object.clone(),
        })),
    );

    instance.set_attribute(
        String::from("to_i"),
        Rc::new(KyaObject::Method(KyaMethod {
            function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                String::from("to_i"),
                kya_string_to_i,
            ))),
            instance: object.clone(),
        })),
    );

    Ok(object)
}

pub fn instantiate_string(
    _: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let value = unpack_string(&args, 0, 1)?.as_string()?.to_string();

    kya_string_new(&value)
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

    let instance = interpreter.get_self()?;

    if instance.name() != "String" {
        return Ok(interpreter.false_object());
    }

    let self_value = instance.as_string()?;
    let other_value = args[0].as_string()?;

    if self_value == other_value {
        return Ok(interpreter.true_object());
    } else {
        return Ok(interpreter.false_object());
    }
}

pub fn kya_string_add(
    interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let other_value = unpack_args(&args, 0, 1)?.as_string()?;
    let self_value = interpreter.get_self()?.as_string()?.to_string();

    Ok(kya_string_new(&(self_value + &other_value)).unwrap())
}

pub fn kya_string_split(
    interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let separator_value = unpack_args(&args, 0, 1)
        .unwrap_or_else(|_| kya_string_new(" ").unwrap())
        .as_string()?
        .to_string();
    let instance = interpreter.get_self()?;
    let value = instance.as_string()?;
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
    let value = instance.as_string()?.to_string();

    match value.parse::<i64>() {
        Ok(num) => Ok(kya_number_new(num as f64).unwrap()),
        Err(_) => Err(Error::RuntimeError(
            "Cannot convert string to integer".to_string(),
        )),
    }
}
