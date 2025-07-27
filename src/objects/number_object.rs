use crate::bytecode::ComparisonOperator;
use crate::errors::Error;

use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef, BASE_TYPE};
use crate::objects::bool_object::{BoolObject, BOOL_TYPE};
use crate::objects::string_object::{StringObject, STRING_TYPE};
use crate::objects::utils::bool_to_bool_object;

use once_cell::sync::Lazy;

pub struct NumberObject {
    pub ob_type: TypeRef,
    pub value: f64,
}

impl KyaObjectTrait for NumberObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn number_tp_repr(
    callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.lock().unwrap();

    if let KyaObject::NumberObject(number) = &*object {
        Ok(KyaObject::from_string_object(StringObject {
            ob_type: STRING_TYPE.clone(),
            value: number.value.to_string(),
        }))
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a number",
            object.get_type()?.lock().unwrap().name
        )))
    }
}

pub fn number_nb_bool(object: KyaObjectRef) -> Result<f64, Error> {
    if let KyaObject::NumberObject(obj) = &*object.lock().unwrap() {
        Ok(if obj.value != 0.0 { 1.0 } else { 0.0 })
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a number",
            object.lock().unwrap().get_type()?.lock().unwrap().name
        )))
    }
}

pub fn number_tp_add(obj1: KyaObjectRef, obj2: KyaObjectRef) -> Result<KyaObjectRef, Error> {
    let a;
    let b;

    if let KyaObject::NumberObject(num1) = &*obj1.lock().unwrap() {
        a = num1.value;
    } else {
        return Err(Error::RuntimeError(format!(
            "Unsupported operand types: '{}' and 'Number'",
            obj1.lock().unwrap().get_type()?.lock().unwrap().name
        )));
    }

    if let KyaObject::NumberObject(num2) = &*obj2.lock().unwrap() {
        b = num2.value;
    } else {
        return Err(Error::RuntimeError(format!(
            "Unsupported operand types: 'Number' and '{}'",
            obj2.lock().unwrap().get_type()?.lock().unwrap().name
        )));
    }

    Ok(number_new(a + b))
}

pub fn number_tp_sub(obj1: KyaObjectRef, obj2: KyaObjectRef) -> Result<KyaObjectRef, Error> {
    let a;
    let b;

    if let KyaObject::NumberObject(num1) = &*obj1.lock().unwrap() {
        a = num1.value;
    } else {
        return Err(Error::RuntimeError(format!(
            "Unsupported operand types: '{}' and 'Number'",
            obj1.lock().unwrap().get_type()?.lock().unwrap().name
        )));
    }

    if let KyaObject::NumberObject(num2) = &*obj2.lock().unwrap() {
        b = num2.value;
    } else {
        return Err(Error::RuntimeError(format!(
            "Unsupported operand types: 'Number' and '{}'",
            obj2.lock().unwrap().get_type()?.lock().unwrap().name
        )));
    }

    Ok(number_new(a - b))
}

pub fn number_tp_compare(
    obj1: KyaObjectRef,
    obj2: KyaObjectRef,
    operator: ComparisonOperator,
) -> Result<KyaObjectRef, Error> {
    let a;
    let b;

    if let KyaObject::NumberObject(num1) = &*obj1.lock().unwrap() {
        a = num1.value;
    } else {
        return Err(Error::RuntimeError(format!(
            "The first object '{}' is not a number",
            obj1.lock().unwrap().get_type()?.lock().unwrap().name
        )));
    }

    if let KyaObject::NumberObject(num2) = &*obj2.lock().unwrap() {
        b = num2.value;
    } else {
        return Err(Error::RuntimeError(format!(
            "The second object '{}' is not a number",
            obj2.lock().unwrap().get_type()?.lock().unwrap().name
        )));
    }

    match operator {
        ComparisonOperator::Equal => Ok(bool_to_bool_object(a == b)),
        ComparisonOperator::Neq => Ok(bool_to_bool_object(a != b)),
        ComparisonOperator::Gt => Ok(bool_to_bool_object(a > b)),
        ComparisonOperator::Lt => Ok(bool_to_bool_object(a < b)),
        ComparisonOperator::Gte => Ok(bool_to_bool_object(a >= b)),
        ComparisonOperator::Lte => Ok(bool_to_bool_object(a <= b)),
    }
}

pub fn number_new(value: f64) -> KyaObjectRef {
    KyaObject::from_number_object(NumberObject {
        ob_type: NUMBER_TYPE.clone(),
        value,
    })
}

pub static NUMBER_TYPE: Lazy<TypeRef> = Lazy::new(|| {
    Type::as_ref(Type {
        ob_type: Some(BASE_TYPE.clone()),
        name: "Number".to_string(),
        tp_repr: Some(number_tp_repr),
        nb_bool: Some(number_nb_bool),
        tp_compare: Some(number_tp_compare),
        tp_add: Some(number_tp_add),
        tp_sub: Some(number_tp_sub),
        ..Default::default()
    })
});
