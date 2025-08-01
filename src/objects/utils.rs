use crate::errors::Error;
use crate::interpreter::{FALSE_OBJECT, NONE_OBJECT, TRUE_OBJECT};
use crate::objects::base::{kya_nb_bool, kya_repr, KyaObject, KyaObjectRef};

pub fn parse_arg(
    args: &Vec<KyaObjectRef>,
    index: usize,
    args_count: usize,
) -> Result<KyaObjectRef, Error> {
    if args_count == 0 && args.is_empty() {
        return Ok(NONE_OBJECT.clone());
    }

    if index >= args_count {
        return Err(Error::RuntimeError(format!(
            "Expected {} arguments, but got {}",
            args_count, index
        )));
    }
    if index < args.len() {
        Ok(args[index].clone())
    } else {
        Err(Error::RuntimeError(format!(
            "Argument at index {} not found",
            index
        )))
    }
}

pub fn string_object_to_string(obj: &KyaObjectRef) -> Result<String, Error> {
    if let KyaObject::StringObject(string_obj) = &*obj.lock().unwrap() {
        Ok(string_obj.value.clone())
    } else {
        Err(Error::RuntimeError("Expected a String".to_string()))
    }
}

pub fn number_object_to_float(obj: &KyaObjectRef) -> Result<f64, Error> {
    if let KyaObject::NumberObject(number_obj) = &*obj.lock().unwrap() {
        Ok(number_obj.value)
    } else {
        Err(Error::RuntimeError("Expected a Number".to_string()))
    }
}

pub fn bool_to_bool_object(value: bool) -> KyaObjectRef {
    if value {
        TRUE_OBJECT.clone()
    } else {
        FALSE_OBJECT.clone()
    }
}

pub fn kya_is_true(obj: KyaObjectRef) -> Result<bool, Error> {
    if kya_nb_bool(obj.clone())? != 0.0 {
        return Ok(true);
    } else {
        return Ok(false);
    }
}

pub fn kya_is_false(obj: KyaObjectRef) -> Result<bool, Error> {
    if kya_nb_bool(obj.clone())? == 0.0 {
        return Ok(true);
    } else {
        return Ok(false);
    }
}

pub fn parse_receiver(receiver: &Option<KyaObjectRef>) -> Result<KyaObjectRef, Error> {
    if let Some(r) = receiver {
        Ok(r.clone())
    } else {
        Err(Error::RuntimeError("Receiver is None".to_string()))
    }
}

pub fn object_to_string_repr(obj: &KyaObjectRef) -> Result<String, Error> {
    let string_object = kya_repr(obj.clone(), &mut vec![], None)?;

    Ok(string_object_to_string(&string_object)?)
}
