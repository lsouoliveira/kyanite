use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::base::{CallableFunctionPtr, KyaObject, KyaObjectRef};
use crate::objects::rs_function_object::RsFunctionObject;

pub fn parse_arg(
    args: &Vec<KyaObjectRef>,
    index: usize,
    args_count: usize,
) -> Result<KyaObjectRef, Error> {
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
    if let KyaObject::StringObject(string_obj) = &*obj.borrow() {
        Ok(string_obj.value.clone())
    } else {
        Err(Error::RuntimeError("Expected a String".to_string()))
    }
}

pub fn create_rs_function_object(
    interpreter: &mut Interpreter,
    function_ptr: CallableFunctionPtr,
) -> KyaObjectRef {
    KyaObject::from_rs_function_object(RsFunctionObject {
        ob_type: interpreter.get_type("RsFunction"),
        function_ptr,
    })
}

pub fn number_object_to_float(obj: &KyaObjectRef) -> Result<f64, Error> {
    if let KyaObject::NumberObject(number_obj) = &*obj.borrow() {
        Ok(number_obj.value)
    } else {
        Err(Error::RuntimeError("Expected a Number".to_string()))
    }
}

pub fn kya_is_true(interpreter: &mut Interpreter, obj: KyaObjectRef) -> Result<bool, Error> {
    if obj
        .borrow()
        .get_type()?
        .borrow()
        .nb_bool(interpreter, obj.clone())?
        != 0.0
    {
        return Ok(true);
    } else {
        return Ok(false);
    }
}
