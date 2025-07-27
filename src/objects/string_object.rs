use crate::bytecode::ComparisonOperator;
use crate::errors::Error;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef, BASE_TYPE};
use crate::objects::bool_object::bool_new;
use crate::objects::list_object::list_new;
use crate::objects::none_object::none_new;
use crate::objects::number_object::number_new;
use crate::objects::rs_function_object::rs_function_new;
use crate::objects::utils::{parse_arg, parse_receiver};
use once_cell::sync::Lazy;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

pub struct StringObject {
    pub ob_type: TypeRef,
    pub value: String,
}

impl KyaObjectTrait for StringObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn string_new(value: &str) -> KyaObjectRef {
    KyaObject::from_string_object(StringObject {
        ob_type: STRING_TYPE.clone(),
        value: value.to_string(),
    })
}

pub fn string_tp_repr(
    callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.lock().unwrap();

    if let KyaObject::StringObject(_) = &*object {
        Ok(callable.clone())
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a string",
            object.get_type()?.lock().unwrap().name
        )))
    }
}

pub fn string_tp_init(
    callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    if args.len() > 1 {
        return Err(Error::RuntimeError(
            "Expected at most one argument".to_string(),
        ));
    }

    let arg = parse_arg(&args, 0, 1)?;

    if let KyaObject::StringObject(arg_string) = &*arg.lock().unwrap() {
        if let KyaObject::StringObject(ref mut object) = *callable.lock().unwrap() {
            object.value = arg_string.value.clone();
        } else {
            return Err(Error::RuntimeError("Expected a string object".to_string()));
        }

        Ok(none_new()?)
    } else {
        Err(Error::RuntimeError("Expected a string object".to_string()))
    }
}

pub fn string_tp_new(
    _ob_type: TypeRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    Ok(string_new(""))
}

pub fn string_length(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let _ = parse_arg(&_args, 0, 0)?;
    let instance = parse_receiver(&receiver)?;

    if let KyaObject::StringObject(string_object) = &*instance.lock().unwrap() {
        Ok(number_new(string_object.value.len() as f64))
    } else {
        Err(Error::RuntimeError("Expected a string object".to_string()))
    }
}

pub fn string_tp_hash(obj: KyaObjectRef) -> Result<usize, Error> {
    let mut hasher = DefaultHasher::new();

    if let KyaObject::StringObject(string_object) = &*obj.lock().unwrap() {
        string_object.value.hash(&mut hasher);
        Ok(hasher.finish() as usize)
    } else {
        Err(Error::RuntimeError("Expected a string object".to_string()))
    }
}

pub fn string_tp_compare(
    obj1: KyaObjectRef,
    obj2: KyaObjectRef,
    _operator: ComparisonOperator,
) -> Result<KyaObjectRef, Error> {
    if let (KyaObject::StringObject(string1), KyaObject::StringObject(string2)) =
        (&*obj1.lock().unwrap(), &*obj2.lock().unwrap())
    {
        Ok(bool_new(string1.value == string2.value))
    } else {
        Err(Error::RuntimeError(
            "Expected both objects to be strings".to_string(),
        ))
    }
}

pub fn string_char_at(
    _callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let index = parse_arg(&args, 0, 1)?;
    let instance = parse_receiver(&receiver)?;

    if let KyaObject::StringObject(string_object) = &*instance.lock().unwrap() {
        if let KyaObject::NumberObject(number_object) = &*index.lock().unwrap() {
            let idx = number_object.value as usize;
            if idx < string_object.value.len() {
                Ok(string_new(&string_object.value[idx..=idx]))
            } else {
                Err(Error::RuntimeError(format!(
                    "Index out of bounds: {} for string of length {}",
                    idx,
                    string_object.value.len()
                )))
            }
        } else {
            Err(Error::TypeError("Expected a number".to_string()))
        }
    } else {
        Err(Error::RuntimeError("Expected a string object".to_string()))
    }
}

pub fn string_split(
    _callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let separator = parse_arg(&args, 0, 1)?;
    let instance = parse_receiver(&receiver)?;

    if let KyaObject::StringObject(string_object) = &*instance.lock().unwrap() {
        if let KyaObject::StringObject(separator_string) = &*separator.lock().unwrap() {
            let parts: Vec<KyaObjectRef> = string_object
                .value
                .split(&separator_string.value)
                .map(|s| string_new(s))
                .collect();

            Ok(list_new(parts))
        } else {
            Err(Error::TypeError("Expected a string".to_string()))
        }
    } else {
        Err(Error::RuntimeError("Expected a string object".to_string()))
    }
}

pub fn string_substr(
    _callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let start = parse_arg(&args, 0, 1)?;
    let end = parse_arg(&args, 1, 2)?;
    let instance = parse_receiver(&receiver)?;

    if let KyaObject::StringObject(string_object) = &*instance.lock().unwrap() {
        if let (KyaObject::NumberObject(start_num), KyaObject::NumberObject(end_num)) =
            (&*start.lock().unwrap(), &*end.lock().unwrap())
        {
            let start_idx = start_num.value as usize;
            let end_idx = end_num.value as usize;

            if start_idx <= end_idx && end_idx <= string_object.value.len() {
                Ok(string_new(&string_object.value[start_idx..end_idx]))
            } else {
                Err(Error::RuntimeError(format!(
                    "Invalid substring range: {} to {} for string of length {}",
                    start_idx,
                    end_idx,
                    string_object.value.len()
                )))
            }
        } else {
            Err(Error::TypeError(
                "Expected numbers for start and end".to_string(),
            ))
        }
    } else {
        Err(Error::RuntimeError("Expected a string object".to_string()))
    }
}

pub static STRING_TYPE: Lazy<TypeRef> = Lazy::new(|| {
    let dict = Arc::new(Mutex::new(HashMap::new()));

    dict.lock()
        .unwrap()
        .insert("length".to_string(), rs_function_new(string_length));

    dict.lock()
        .unwrap()
        .insert("char_at".to_string(), rs_function_new(string_char_at));

    dict.lock()
        .unwrap()
        .insert("split".to_string(), rs_function_new(string_split));

    dict.lock()
        .unwrap()
        .insert("substr".to_string(), rs_function_new(string_substr));

    Type::as_ref(Type {
        ob_type: Some(BASE_TYPE.clone()),
        name: "String".to_string(),
        tp_repr: Some(string_tp_repr),
        tp_new: Some(string_tp_new),
        tp_init: Some(string_tp_init),
        tp_compare: Some(string_tp_compare),
        tp_hash: Some(string_tp_hash),
        ..Default::default()
    })
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_new() {
        let string = string_new("Hello, World!");

        assert_eq!(
            string
                .lock()
                .unwrap()
                .get_type()
                .unwrap()
                .lock()
                .unwrap()
                .name,
            "String"
        );

        if let KyaObject::StringObject(string_object) = &*string.lock().unwrap() {
            assert_eq!(string_object.value, "Hello, World!");
        } else {
            panic!("Expected a StringObject");
        }
    }

    #[test]
    fn test_string_length() {
        let string = string_new("Hello, World!");
        let length = string_length(string.clone(), &mut vec![], Some(string.clone()));

        assert!(length.is_ok());
        if let Ok(length_obj) = length {
            if let KyaObject::NumberObject(number_object) = &*length_obj.lock().unwrap() {
                assert_eq!(number_object.value, 13.0);
            } else {
                panic!("Expected a NumberObject");
            }
        }
    }

    #[test]
    fn test_string_char_at() {
        let string = string_new("Hello, World!");
        let char_at = string_char_at(
            string.clone(),
            &mut vec![number_new(7.0)],
            Some(string.clone()),
        );

        assert!(char_at.is_ok());
        if let Ok(char_obj) = char_at {
            if let KyaObject::StringObject(string_object) = &*char_obj.lock().unwrap() {
                assert_eq!(string_object.value, "W");
            } else {
                panic!("Expected a StringObject");
            }
        }
    }

    #[test]
    fn test_string_split() {
        let string = string_new("Hello, World!");
        let split_result = string_split(
            string.clone(),
            &mut vec![string_new(", ")],
            Some(string.clone()),
        );

        assert!(split_result.is_ok());
        if let Ok(list_obj) = split_result {
            if let KyaObject::ListObject(list_object) = &*list_obj.lock().unwrap() {
                assert_eq!(list_object.items.len(), 2);
            } else {
                panic!("Expected a ListObject");
            }
        }
    }

    #[test]
    fn test_string_substr() {
        let string = string_new("Hello, World!");
        let substr_result = string_substr(
            string.clone(),
            &mut vec![number_new(7.0), number_new(12.0)],
            Some(string.clone()),
        );

        assert!(substr_result.is_ok());
        if let Ok(substr_obj) = substr_result {
            if let KyaObject::StringObject(string_object) = &*substr_obj.lock().unwrap() {
                assert_eq!(string_object.value, "World");
            } else {
                panic!("Expected a StringObject");
            }
        }
    }
}
