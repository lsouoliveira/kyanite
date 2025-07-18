use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef};
use crate::objects::string_object::StringObject;

pub static BOOL_TYPE: &str = "Bool";

pub struct BoolObject {
    pub ob_type: TypeRef,
    pub value: bool,
}

impl KyaObjectTrait for BoolObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn create_bool_type(ob_type: TypeRef) -> TypeRef {
    Type::as_ref(Type {
        ob_type: Some(ob_type.clone()),
        name: BOOL_TYPE.to_string(),
        tp_repr: Some(bool_tp_repr),
        ..Default::default()
    })
}

pub fn bool_tp_repr(
    _interpreter: &mut Interpreter,
    callable: KyaObjectRef,
    _args: Vec<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.borrow();

    if let KyaObject::BoolObject(obj) = &*object {
        let repr = if obj.value {
            "true".to_string()
        } else {
            "false".to_string()
        };

        Ok(KyaObject::from_string_object(StringObject {
            ob_type: _interpreter.get_type("String"),
            value: repr,
        }))
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a string",
            object.get_type()?.borrow().name
        )))
    }
}
