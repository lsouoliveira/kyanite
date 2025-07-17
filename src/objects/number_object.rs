use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef};
use crate::objects::string_object::StringObject;

pub struct NumberObject {
    pub ob_type: TypeRef,
    pub value: f64,
}

impl KyaObjectTrait for NumberObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn create_number_type(ob_type: TypeRef) -> TypeRef {
    Type::as_ref(Type {
        ob_type: Some(ob_type.clone()),
        name: "Number".to_string(),
        tp_repr: Some(number_tp_repr),
        ..Default::default()
    })
}

pub fn number_tp_repr(
    interpreter: &mut Interpreter,
    callable: KyaObjectRef,
    _args: Vec<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.borrow();

    if let KyaObject::NumberObject(number) = &*object {
        Ok(KyaObject::from_string_object(StringObject {
            ob_type: interpreter.get_type("String"),
            value: number.value.to_string(),
        }))
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a number",
            object.get_type()?.borrow().name
        )))
    }
}
