use crate::errors::Error;
use crate::interpreter::NONE_OBJECT;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef, BASE_TYPE};
use crate::objects::number_object::number_new;
use crate::objects::rs_function_object::rs_function_new;
use crate::objects::string_object::string_new;
use crate::objects::utils::{parse_arg, parse_receiver};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use url::Url;

pub struct UrlObject {
    pub ob_type: TypeRef,
    pub url: Url,
}

impl KyaObjectTrait for UrlObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn url_new(url: Url) -> KyaObjectRef {
    KyaObject::from_url_object(UrlObject {
        ob_type: URL_TYPE.clone(),
        url,
    })
}

pub fn url_tp_init(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    Ok(NONE_OBJECT.clone())
}

pub fn url_tp_new(
    _ob_type: TypeRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    Err(Error::TypeError(
        "Url object cannot be instantiated directly".to_string(),
    ))
}

pub fn url_parse(
    _callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let url_str = parse_arg(args, 0, 1)?;

    if let KyaObject::StringObject(obj) = &*url_str.lock().unwrap() {
        Url::parse(&obj.value)
            .map(|url| url_new(url))
            .map_err(|e| Error::ValueError(format!("Invalid URL: {}", e)))
    } else {
        Err(Error::TypeError(
            "Expected a string argument for URL parsing".to_string(),
        ))
    }
}

pub fn url_tp_repr(
    callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    if let KyaObject::UrlObject(obj) = &*callable.lock().unwrap() {
        Ok(string_new(&obj.url.as_str().to_string()))
    } else {
        Err(Error::TypeError(
            "Expected a Url object for repr".to_string(),
        ))
    }
}

pub fn url_scheme(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let instance = parse_receiver(&receiver)?;

    if let KyaObject::UrlObject(obj) = &*instance.lock().unwrap() {
        Ok(string_new(&obj.url.scheme().to_string()))
    } else {
        Err(Error::TypeError(
            "Expected a Url object for scheme".to_string(),
        ))
    }
}

pub fn url_host(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let instance = parse_receiver(&receiver)?;

    if let KyaObject::UrlObject(obj) = &*instance.lock().unwrap() {
        Ok(string_new(&obj.url.host_str().unwrap_or("").to_string()))
    } else {
        Err(Error::TypeError(
            "Expected a Url object for host".to_string(),
        ))
    }
}

pub fn url_port(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let instance = parse_receiver(&receiver)?;

    if let KyaObject::UrlObject(obj) = &*instance.lock().unwrap() {
        if let Some(port) = obj.url.port() {
            Ok(number_new(port as f64))
        } else {
            Ok(NONE_OBJECT.clone())
        }
    } else {
        Err(Error::TypeError(
            "Expected a Url object for port".to_string(),
        ))
    }
}

pub fn url_path(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let instance = parse_receiver(&receiver)?;

    if let KyaObject::UrlObject(obj) = &*instance.lock().unwrap() {
        Ok(string_new(&obj.url.path().to_string()))
    } else {
        Err(Error::TypeError(
            "Expected a Url object for path".to_string(),
        ))
    }
}

pub fn url_query(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let instance = parse_receiver(&receiver)?;

    if let KyaObject::UrlObject(obj) = &*instance.lock().unwrap() {
        if let Some(query) = obj.url.query() {
            Ok(string_new(&query.to_string()))
        } else {
            Ok(string_new(""))
        }
    } else {
        Err(Error::TypeError(
            "Expected a Url object for query".to_string(),
        ))
    }
}

pub static URL_TYPE: Lazy<TypeRef> = Lazy::new(|| {
    let dict = Arc::new(Mutex::new(HashMap::new()));

    dict.lock()
        .unwrap()
        .insert("parse".to_string(), rs_function_new(url_parse));

    dict.lock()
        .unwrap()
        .insert("scheme".to_string(), rs_function_new(url_scheme));

    dict.lock()
        .unwrap()
        .insert("host".to_string(), rs_function_new(url_host));

    dict.lock()
        .unwrap()
        .insert("port".to_string(), rs_function_new(url_port));

    dict.lock()
        .unwrap()
        .insert("path".to_string(), rs_function_new(url_path));

    dict.lock()
        .unwrap()
        .insert("query".to_string(), rs_function_new(url_query));

    Type::as_ref(Type {
        ob_type: Some(BASE_TYPE.clone()),
        name: "Url".to_string(),
        tp_new: Some(url_tp_new),
        tp_init: Some(url_tp_init),
        tp_repr: Some(url_tp_repr),
        dict: dict,
        ..Default::default()
    })
});
