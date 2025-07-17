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
