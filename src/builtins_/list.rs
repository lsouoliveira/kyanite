use crate::builtins_::number::kya_number_new;
use crate::builtins_::string::kya_string_new;
use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::{Context, KyaInstanceObject, KyaList, KyaMethod, KyaObject, KyaRsFunction};

use std::cell::RefCell;
use std::rc::Rc;

pub fn kya_list_repr(
    interpreter: &mut Interpreter,
    _: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let instance = interpreter.get_self()?;
    let items = instance.as_vector()?;
    let mut output = String::new();

    output.push('[');

    for (i, value) in items.iter().enumerate() {
        let repr = value.get_attribute("__repr__")?.call(interpreter, vec![])?;

        output.push_str(repr.as_string()?.as_str());

        if i < items.len() - 1 {
            output.push_str(", ");
        }
    }

    output.push(']');

    Ok(kya_string_new(&output)?)
}

pub fn kya_list_length(
    interpreter: &mut Interpreter,
    _: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let instance = interpreter.get_self()?;
    let items = instance.as_vector()?;

    return Ok(kya_number_new(items.len() as f64)?);
}

pub fn kya_list_new(items: Vec<Rc<KyaObject>>) -> Result<Rc<KyaObject>, Error> {
    let instance = Rc::new(KyaObject::InstanceObject(KyaInstanceObject::new(
        "List".to_string(),
        RefCell::new(Context::new()),
    )));

    let object = Rc::new(KyaObject::List(KyaList {
        items: RefCell::new(items),
        instance: instance.clone(),
    }));

    instance.set_attribute(
        String::from("__repr__"),
        Rc::new(KyaObject::Method(KyaMethod {
            function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                String::from("__repr__"),
                kya_list_repr,
            ))),
            instance: object.clone(),
        })),
    );

    instance.set_attribute(
        String::from("length"),
        Rc::new(KyaObject::Method(KyaMethod {
            function: Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
                String::from("length"),
                kya_list_length,
            ))),
            instance: object.clone(),
        })),
    );

    Ok(object)
}
