use crate::interpreter::Interpreter;
use crate::objects::base::{DictRef, KyaObjectTrait, Type, TypeRef};

pub struct ClassObject {
    pub ob_type: TypeRef,
}

impl KyaObjectTrait for ClassObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn create_class_type(interpreter: &mut Interpreter, name: String, dict: DictRef) -> TypeRef {
    Type::as_ref(Type {
        ob_type: Some(interpreter.get_type("Type")),
        name: name.clone(),
        dict: dict.clone(),
        ..Default::default()
    })
}
