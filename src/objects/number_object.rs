use crate::ast;
use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef};
use crate::objects::bool_object::{BoolObject, BOOL_TYPE};
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
        nb_bool: Some(number_nb_bool),
        tp_compare: Some(number_tp_compare),
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

pub fn number_nb_bool(_interpreter: &mut Interpreter, object: KyaObjectRef) -> Result<f64, Error> {
    if let KyaObject::NumberObject(obj) = &*object.borrow() {
        Ok(if obj.value != 0.0 { 1.0 } else { 0.0 })
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a number",
            object.borrow().get_type()?.borrow().name
        )))
    }
}

pub fn number_tp_compare(
    interpreter: &mut Interpreter,
    obj1: KyaObjectRef,
    obj2: KyaObjectRef,
    _operator: ast::Operator,
) -> Result<KyaObjectRef, Error> {
    if let (KyaObject::NumberObject(num1), KyaObject::NumberObject(num2)) =
        (&*obj1.borrow(), &*obj2.borrow())
    {
        return Ok(KyaObject::from_bool_object(BoolObject {
            ob_type: interpreter.get_type(BOOL_TYPE),
            value: num1.value == num2.value,
        }));
    } else {
        return Err(Error::RuntimeError(format!(
            "Cannot compare '{}' with '{}'",
            obj1.borrow().get_type()?.borrow().name,
            obj2.borrow().get_type()?.borrow().name
        )));
    }
}

pub fn kya_compare_numbers(
    interpreter: &mut Interpreter,
    obj1: KyaObjectRef,
    obj2: KyaObjectRef,
    operator: ast::Operator,
) -> Result<KyaObjectRef, Error> {
    obj1.borrow()
        .get_type()?
        .borrow()
        .tp_compare(interpreter, obj1.clone(), obj2, operator)
        .map_err(|e| Error::RuntimeError(format!("Error comparing numbers: {}", e)))
}
