use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::base::{
    CallableFunctionPtr, KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef,
};

pub struct RsFunctionObject {
    pub ob_type: TypeRef,
    pub function_ptr: CallableFunctionPtr,
}

impl RsFunctionObject {
    pub fn new(ob_type: TypeRef, function_ptr: CallableFunctionPtr) -> Self {
        Self {
            ob_type,
            function_ptr,
        }
    }
}

impl KyaObjectTrait for RsFunctionObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn create_rs_function_type(ob_type: TypeRef) -> TypeRef {
    Type::as_ref(Type {
        ob_type: Some(ob_type.clone()),
        name: "RsFunction".to_string(),
        tp_call: Some(rs_function_tp_call),
        ..Default::default()
    })
}

pub fn rs_function_tp_call(
    interpreter: &mut Interpreter,
    callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.borrow();

    if let KyaObject::RsFunctionObject(rs_function) = &*object {
        interpreter.push_next_frame();

        if let Some(receiver) = receiver.clone() {
            interpreter.register("self", receiver);
        }

        let result = (rs_function.function_ptr)(interpreter, callable.clone(), args, receiver);

        interpreter.pop_frame();

        result
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not callable",
            object.get_type()?.borrow().name
        )))
    }
}
