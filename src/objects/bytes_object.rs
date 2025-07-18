use crate::errors::Error;
use crate::interpreter::{Interpreter, STRING_TYPE};
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef};
use crate::objects::string_object::StringObject;

pub struct BytesObject {
    pub ob_type: TypeRef,
    pub value: Vec<u8>,
}

impl KyaObjectTrait for BytesObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn create_bytes_type(ob_type: TypeRef) -> TypeRef {
    Type::as_ref(Type {
        ob_type: Some(ob_type.clone()),
        name: "Bytes".to_string(),
        tp_repr: Some(bytes_tp_repr),
        ..Default::default()
    })
}

pub fn bytes_tp_repr(
    interpreter: &mut Interpreter,
    callable: KyaObjectRef,
    _args: Vec<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.borrow();

    if let KyaObject::BytesObject(obj) = &*object {
        Ok(KyaObject::from_string_object(StringObject {
            ob_type: interpreter.get_type(STRING_TYPE),
            value: format!("b'{}'", String::from_utf8_lossy(&obj.value)),
        }))
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a bytes object.",
            object.get_type()?.borrow().name
        )))
    }
}
