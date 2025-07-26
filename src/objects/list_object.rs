use crate::errors::Error;
use crate::interpreter::NONE_OBJECT;
use crate::objects::base::{
    kya_call, kya_init, kya_repr, KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef, BASE_TYPE,
};
use crate::objects::rs_function_object::rs_function_new;
use crate::objects::string_object::string_new;
use crate::objects::utils::{parse_arg, parse_receiver, string_object_to_string};
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

        for item in &obj.items {
            let repr = kya_repr(item.clone(), &mut vec![], None)?;
            let repr_str = string_object_to_string(&repr)?;

            output.push_str(&repr_str);
        }

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

pub static LIST_TYPE: Lazy<TypeRef> = Lazy::new(|| {
    let dict = Arc::new(Mutex::new(HashMap::new()));

    dict.lock()
        .unwrap()
        .insert("append".to_string(), rs_function_new(list_append));

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
