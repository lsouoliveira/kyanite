use crate::bytecode::ComparisonOperator;
use crate::errors::Error;
use crate::interpreter::NONE_OBJECT;
use crate::objects::base::{
    kya_compare, kya_hash, kya_init, kya_repr, KyaObject, KyaObjectRef, KyaObjectTrait, Type,
    TypeRef, BASE_TYPE,
};
use crate::objects::rs_function_object::rs_function_new;
use crate::objects::string_object::string_new;
use crate::objects::utils::{kya_is_true, parse_arg, parse_receiver, string_object_to_string};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct HashObject {
    pub ob_type: TypeRef,
    pub items: Arc<Mutex<HashMap<usize, HashItem>>>,
}

#[derive(Clone)]
struct HashItem {
    key: KyaObjectRef,
    value: KyaObjectRef,
}

impl KyaObjectTrait for HashObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn hash_new(items: HashMap<usize, HashItem>) -> KyaObjectRef {
    KyaObject::from_hash_object(HashObject {
        ob_type: HASH_TYPE.clone(),
        items: Arc::new(Mutex::new(items)),
    })
}

pub fn hash_tp_new(
    _ob_type: TypeRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let obj = hash_new(HashMap::new());

    kya_init(obj.clone(), _args, _receiver)?;

    Ok(obj)
}

pub fn hash_tp_init(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    Ok(NONE_OBJECT.clone())
}

pub fn hash_tp_repr(
    callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let items = match &*callable.lock().unwrap() {
        KyaObject::HashObject(hash) => hash.items.clone(),
        _ => {
            return Err(Error::RuntimeError(format!(
                "The object '{}' is not a hash",
                callable.lock().unwrap().get_type()?.lock().unwrap().name
            )))
        }
    };

    let mut output = String::from("{");

    for (_, item) in items.lock().unwrap().iter() {
        let key_repr = string_object_to_string(&item.key)?;
        let value_repr = string_object_to_string(&item.value)?;

        output.push_str(&format!("{}: {}, ", key_repr, value_repr));
    }

    if output.ends_with(", ") {
        output.truncate(output.len() - 2);
    }

    output.push('}');

    Ok(string_new(&output))
}

pub fn hash_get(
    _callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let key = parse_arg(args, 0, 1)?;
    let instance = parse_receiver(&receiver)?;
    let items = match &*instance.lock().unwrap() {
        KyaObject::HashObject(hash) => hash.items.clone(),
        _ => {
            return Err(Error::RuntimeError(format!(
                "The object '{}' is not a hash",
                instance.lock().unwrap().get_type()?.lock().unwrap().name
            )))
        }
    };
    let key_hash = kya_hash(key.clone())?;
    let item = items.lock().unwrap().get(&key_hash).cloned();

    if let Some(item) = item {
        let compare_result = kya_compare(item.key.clone(), key, ComparisonOperator::Equal)?;

        if kya_is_true(compare_result)? {
            return Ok(item.value);
        }
    }

    Ok(NONE_OBJECT.clone())
}

pub fn hash_insert(
    _callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let key = parse_arg(args, 0, 1)?;
    let value = parse_arg(args, 1, 2)?;
    let instance = parse_receiver(&receiver)?;
    let items = match &*instance.lock().unwrap() {
        KyaObject::HashObject(hash) => hash.items.clone(),
        _ => {
            return Err(Error::RuntimeError(format!(
                "The object '{}' is not a hash",
                instance.lock().unwrap().get_type()?.lock().unwrap().name
            )))
        }
    };
    let key_hash = kya_hash(key.clone())?;

    items.lock().unwrap().insert(
        key_hash,
        HashItem {
            key: key.clone(),
            value: value.clone(),
        },
    );

    Ok(NONE_OBJECT.clone())
}

pub static HASH_TYPE: Lazy<TypeRef> = Lazy::new(|| {
    let dict = Arc::new(Mutex::new(HashMap::new()));

    dict.lock()
        .unwrap()
        .insert("get".to_string(), rs_function_new(hash_get));

    dict.lock()
        .unwrap()
        .insert("insert".to_string(), rs_function_new(hash_insert));

    Type::as_ref(Type {
        ob_type: Some(BASE_TYPE.clone()),
        name: "Hash".to_string(),
        tp_new: Some(hash_tp_new),
        tp_init: Some(hash_tp_init),
        tp_repr: Some(hash_tp_repr),
        dict,
        ..Default::default()
    })
});
