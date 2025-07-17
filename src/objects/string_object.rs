use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef};
use crate::objects::utils::{parse_arg, string_object_to_string};

pub struct StringObject {
    pub ob_type: TypeRef,
    pub value: String,
}

impl KyaObjectTrait for StringObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn create_string_type(ob_type: TypeRef) -> TypeRef {
    Type::as_ref(Type {
        ob_type: Some(ob_type.clone()),
        name: "String".to_string(),
        tp_repr: Some(string_tp_repr),
        ..Default::default()
    })
}

pub fn string_new(
    _interpreter: &mut Interpreter,
    ob_type: TypeRef,
    args: Vec<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let arg = parse_arg(&args, 0, 1)?;

    Ok(KyaObject::from_string_object(StringObject {
        ob_type,
        value: string_object_to_string(&arg)?,
    }))
}

pub fn string_tp_repr(
    _interpreter: &mut Interpreter,
    callable: KyaObjectRef,
    _args: Vec<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.borrow();

    if let KyaObject::StringObject(_) = &*object {
        Ok(callable.clone())
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a string",
            object.get_type()?.borrow().name
        )))
    }
}
