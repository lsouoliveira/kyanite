use crate::builtins_::string::kya_string_new;
use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::{
    kya_list_as_vec, kya_string_as_string, Context, KyaInstanceObject, KyaList, KyaObject,
    KyaRsFunction,
};
use std::rc::Rc;

pub fn kya_list_repr(
    interpreter: &mut Interpreter,
    _: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let instance = interpreter.get_self()?;
    let values = kya_list_as_vec(&instance)?;
    let mut output = String::new();

    for (_, value) in values.iter().enumerate() {
        if let KyaObject::InstanceObject(_) = value.as_ref() {
            let repr = interpreter.call_instance_method(value.clone(), "__repr__", vec![])?;

            output.push_str(&kya_string_as_string(&repr)?);
        } else {
            output.push_str(&value.repr());
        }
    }

    Ok(kya_string_new(&output)?)
}

pub fn kya_list_length(
    interpreter: &mut Interpreter,
    _: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let instance = interpreter.get_self()?;

    if let KyaObject::InstanceObject(obj) = instance.as_ref() {
        if let Some(items) = obj.get_attribute("__items__") {
            if let KyaObject::List(list) = items.as_ref() {
                return Ok(Rc::new(KyaObject::Number(list.len() as f64)));
            }
        }
    }

    Err(Error::RuntimeError(
        "List object does not have a __items__ attribute".to_string(),
    ))
}

pub fn kya_list_new(items: Vec<Rc<KyaObject>>) -> Result<Rc<KyaObject>, Error> {
    let mut locals = Context::new();

    locals.register(
        String::from("__items__"),
        Rc::new(KyaObject::List(KyaList::new(items))),
    );

    locals.register(
        String::from("__repr__"),
        Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
            String::from("__repr__"),
            kya_list_repr,
        ))),
    );

    locals.register(
        String::from("length"),
        Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
            String::from("length"),
            kya_list_length,
        ))),
    );

    Ok(Rc::new(KyaObject::InstanceObject(KyaInstanceObject::new(
        "List".to_string(),
        locals,
    ))))
}
