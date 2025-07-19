use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef};
use crate::objects::string_object::StringObject;

pub struct NoneObject {
    ob_type: TypeRef,
}

impl KyaObjectTrait for NoneObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn create_none_type(ob_type: TypeRef) -> TypeRef {
    Type::as_ref(Type {
        ob_type: Some(ob_type.clone()),
        name: "None".to_string(),
        tp_repr: Some(none_repr),
        ..Default::default()
    })
}

pub fn none_new(
    _interpreter: &mut Interpreter,
    ob_type: TypeRef,
    _args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    Ok(KyaObject::from_none_object(NoneObject { ob_type }))
}

pub fn none_repr(
    _interpreter: &mut Interpreter,
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    Ok(KyaObject::from_string_object(StringObject {
        ob_type: _interpreter.get_type("String"),
        value: "None".to_string(),
    }))
}
