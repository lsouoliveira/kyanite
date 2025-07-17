use crate::builtins_::string::kya_string_new;
use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::{
    unpack_args, Context, KyaInstanceObject, KyaMethod, KyaNumber, KyaObject, KyaRsFunction,
};
use std::cell::RefCell;
use std::rc::Rc;

pub fn kya_number_repr(
    interpreter: &mut Interpreter,
    _: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let instance = interpreter.get_self()?;

    Ok(kya_string_new(&instance.as_number()?.to_string())?)
}

pub fn kya_number_eq(
    interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let arg = unpack_args(&args, 0, 1)?;
    let instance = interpreter.get_self()?;

    if instance.name() != "Number" {
        return Ok(interpreter.false_object());
    }

    let self_value = instance.as_number()?;
    let other_value = arg.as_number()?;

    if self_value == other_value {
        return Ok(interpreter.true_object());
    } else {
        return Ok(interpreter.false_object());
    }
}

pub fn kya_number_add(
    interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let arg = unpack_args(&args, 0, 1)?;
    let instance = interpreter.get_self()?;
    let other_value = arg.as_number()?;
    let self_value = instance.as_number()?;

    Ok(kya_number_new(self_value + other_value).unwrap())
}

pub fn kya_number_new(value: f64) -> Result<Rc<KyaObject>, Error> {
    let instance = Rc::new(KyaObject::InstanceObject(KyaInstanceObject::new(
        "Number".to_string(),
        RefCell::new(Context::new()),
    )));

    let object = Rc::new(KyaObject::Number(KyaNumber {
        value: RefCell::new(value),
        instance: instance.clone(),
    }));

    instance.set_attribute(
        "__repr__".to_string(),
        Rc::new(KyaObject::Method(KyaMethod {
            function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                "__repr__".to_string(),
                kya_number_repr,
            ))),
            instance: object.clone(),
        })),
    );

    instance.set_attribute(
        "__eq__".to_string(),
        Rc::new(KyaObject::Method(KyaMethod {
            function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                "__eq__".to_string(),
                kya_number_eq,
            ))),
            instance: object.clone(),
        })),
    );

    instance.set_attribute(
        "__add__".to_string(),
        Rc::new(KyaObject::Method(KyaMethod {
            function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                "__add__".to_string(),
                kya_number_add,
            ))),
            instance: object.clone(),
        })),
    );

    instance.set_attribute(
        "to_s".to_string(),
        Rc::new(KyaObject::Method(KyaMethod {
            function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                "to_s".to_string(),
                kya_number_to_s,
            ))),
            instance: object.clone(),
        })),
    );

    instance.set_attribute(
        "__neg__".to_string(),
        Rc::new(KyaObject::Method(KyaMethod {
            function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                "__neg__".to_string(),
                kya_number_to_neg,
            ))),
            instance: object.clone(),
        })),
    );

    Ok(object)
}

pub fn kya_number_to_s(
    interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    if args.len() != 0 {
        return Err(Error::TypeError(
            "to_s() does not take any arguments".to_string(),
        ));
    }

    let value = interpreter.get_self()?.as_number()?;
    kya_string_new(&value.to_string())
}

pub fn kya_number_to_neg(
    interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    if args.len() != 0 {
        return Err(Error::TypeError(
            "to_neg() does not take any arguments".to_string(),
        ));
    }

    kya_number_new(-interpreter.get_self()?.as_number()?)
}
