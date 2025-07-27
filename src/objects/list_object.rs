use crate::bytecode::ComparisonOperator;
use crate::errors::Error;
use crate::interpreter::NONE_OBJECT;
use crate::objects::base::{
    kya_compare, kya_init, kya_repr, KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef,
    BASE_TYPE,
};
use crate::objects::number_object::number_new;
use crate::objects::rs_function_object::rs_function_new;
use crate::objects::string_object::string_new;
use crate::objects::utils::{kya_is_true, parse_arg, parse_receiver, string_object_to_string};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ListObject {
    pub ob_type: TypeRef,
    pub items: Vec<KyaObjectRef>,
}

impl KyaObjectTrait for ListObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn list_new(items: Vec<KyaObjectRef>) -> KyaObjectRef {
    KyaObject::from_list_object(ListObject {
        ob_type: LIST_TYPE.clone(),
        items: items,
    })
}

pub fn list_tp_new(
    _ob_type: TypeRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let obj = list_new(vec![]);

    kya_init(obj.clone(), _args, _receiver)?;

    Ok(obj)
}

pub fn list_tp_init(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    Ok(NONE_OBJECT.clone())
}

pub fn list_tp_repr(
    callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.lock().unwrap();

    if let KyaObject::ListObject(obj) = &*object {
        let mut output = String::new();

        output.push('[');

        for item in &obj.items {
            let repr = kya_repr(item.clone(), &mut vec![], None)?;
            let repr_str = string_object_to_string(&repr)?;

            output.push_str(&format!("{}, ", repr_str));
        }

        if output.ends_with(", ") {
            output.truncate(output.len() - 2); // Remove the last comma and space
        }

        output.push(']');

        Ok(string_new(&output))
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a string",
            object.get_type()?.lock().unwrap().name
        )))
    }
}

pub fn list_append(
    _callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let instance = parse_receiver(&receiver)?;
    let arg = parse_arg(&args, 0, 1)?;

    if let KyaObject::ListObject(ref mut list_object) = *instance.lock().unwrap() {
        list_object.items.push(arg.clone());

        Ok(instance.clone())
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a list",
            instance.lock().unwrap().get_type()?.lock().unwrap().name
        )))
    }
}

pub fn list_remove(
    _callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let instance = parse_receiver(&receiver)?;
    let arg = parse_arg(&args, 0, 1)?;
    let items = if let KyaObject::ListObject(list_object) = &*instance.lock().unwrap() {
        list_object.items.clone()
    } else {
        return Err(Error::RuntimeError(format!(
            "The object '{}' is not a list",
            instance.lock().unwrap().get_type()?.lock().unwrap().name
        )));
    };

    for (i, item) in items.iter().enumerate() {
        let compare_result = kya_compare(item.clone(), arg.clone(), ComparisonOperator::Equal)?;

        if kya_is_true(compare_result.clone())? {
            if let KyaObject::ListObject(ref mut list_object) = *instance.lock().unwrap() {
                list_object.items.remove(i);
            } else {
                return Err(Error::RuntimeError(format!(
                    "The object '{}' is not a list",
                    instance.lock().unwrap().get_type()?.lock().unwrap().name
                )));
            }

            break;
        }
    }

    Ok(NONE_OBJECT.clone())
}

pub fn list_at(
    _callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let instance = parse_receiver(&receiver)?;
    let index = parse_arg(&args, 0, 1)?;

    if let KyaObject::ListObject(list_object) = &*instance.lock().unwrap() {
        if let KyaObject::NumberObject(index_number) = &*index.lock().unwrap() {
            let idx = index_number.value as usize;

            if idx < list_object.items.len() {
                return Ok(list_object.items[idx].clone());
            } else {
                return Err(Error::RuntimeError(format!("Index out of range: {}", idx)));
            }
        } else {
            return Err(Error::TypeError("Index must be a number".to_string()));
        }
    } else {
        return Err(Error::RuntimeError(format!(
            "The object '{}' is not a list",
            instance.lock().unwrap().get_type()?.lock().unwrap().name
        )));
    }
}

pub fn list_length(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let instance = parse_receiver(&receiver)?;

    if let KyaObject::ListObject(list_object) = &*instance.lock().unwrap() {
        Ok(number_new(list_object.items.len() as f64))
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a list",
            instance.lock().unwrap().get_type()?.lock().unwrap().name
        )))
    }
}

pub static LIST_TYPE: Lazy<TypeRef> = Lazy::new(|| {
    let dict = Arc::new(Mutex::new(HashMap::new()));

    dict.lock()
        .unwrap()
        .insert("append".to_string(), rs_function_new(list_append));

    dict.lock()
        .unwrap()
        .insert("remove".to_string(), rs_function_new(list_remove));

    dict.lock()
        .unwrap()
        .insert("at".to_string(), rs_function_new(list_at));

    dict.lock()
        .unwrap()
        .insert("length".to_string(), rs_function_new(list_length));

    Type::as_ref(Type {
        ob_type: Some(BASE_TYPE.clone()),
        name: "List".to_string(),
        tp_new: Some(list_tp_new),
        tp_init: Some(list_tp_init),
        tp_repr: Some(list_tp_repr),
        dict,
        ..Default::default()
    })
});

pub fn list_slice(
    _callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let instance = parse_receiver(&receiver)?;
    let start = parse_arg(&args, 0, 1)?;
    let end = parse_arg(&args, 1, 2)?;

    if let KyaObject::ListObject(list_object) = &*instance.lock().unwrap() {
        if let (KyaObject::NumberObject(start_num), KyaObject::NumberObject(end_num)) =
            (&*start.lock().unwrap(), &*end.lock().unwrap())
        {
            let start_idx = start_num.value as usize;
            let end_idx = end_num.value as usize;

            if start_idx < list_object.items.len() && end_idx <= list_object.items.len() {
                let slice_items = list_object.items[start_idx..end_idx].to_vec();
                return Ok(list_new(slice_items));
            } else {
                return Err(Error::RuntimeError(format!(
                    "Slice indices out of range: {} to {}",
                    start_idx, end_idx
                )));
            }
        } else {
            return Err(Error::TypeError(
                "Start and end must be numbers".to_string(),
            ));
        }
    } else {
        return Err(Error::RuntimeError(format!(
            "The object '{}' is not a list",
            instance.lock().unwrap().get_type()?.lock().unwrap().name
        )));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::number_object::number_new;

    #[test]
    fn test_list_append() {
        let list = list_new(vec![]);
        list_append(
            list.clone(),
            &mut vec![number_new(42.0)],
            Some(list.clone()),
        )
        .unwrap();

        if let KyaObject::ListObject(list_object) = &*list.lock().unwrap() {
            assert_eq!(list_object.items.len(), 1);
        } else {
            panic!("Expected a ListObject");
        }
    }

    #[test]
    fn test_list_remove() {
        let list = list_new(vec![number_new(42.0), number_new(43.0)]);
        list_remove(
            list.clone(),
            &mut vec![number_new(42.0)],
            Some(list.clone()),
        )
        .unwrap();

        if let KyaObject::ListObject(list_object) = &*list.lock().unwrap() {
            assert_eq!(list_object.items.len(), 1);
        } else {
            panic!("Expected a ListObject");
        }
    }

    #[test]
    fn test_list_at() {
        let list = list_new(vec![number_new(42.0), number_new(43.0)]);
        let item = list_at(list.clone(), &mut vec![number_new(0.0)], Some(list.clone())).unwrap();

        if let KyaObject::NumberObject(num) = &*item.lock().unwrap() {
            assert_eq!(num.value, 42.0);
        } else {
            panic!("Expected a NumberObject");
        }
    }

    #[test]
    fn test_list_length() {
        let list = list_new(vec![number_new(42.0), number_new(43.0)]);
        let length = list_length(list.clone(), &mut vec![], Some(list.clone())).unwrap();

        if let KyaObject::NumberObject(num) = &*length.lock().unwrap() {
            assert_eq!(num.value, 2.0);
        } else {
            panic!("Expected a NumberObject");
        }
    }
}
