use crate::errors::Error;
use crate::interpreter::NONE_OBJECT;
use crate::lock::{kya_acquire_lock, kya_release_lock};
use crate::objects::base::{
    kya_call, KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef, BASE_TYPE,
};
use crate::objects::rs_function_object::rs_function_new;
use crate::objects::string_object::string_new;
use crate::objects::utils::parse_arg;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadObject {
    pub ob_type: TypeRef,
    pub target: KyaObjectRef,
    pub thread_handle: Option<thread::JoinHandle<Result<KyaObjectRef, Error>>>,
}

impl KyaObjectTrait for ThreadObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn thread_tp_repr(
    callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.lock().unwrap();

    if let KyaObject::ThreadObject(_) = &*object {
        Ok(string_new(&format!(
            "<{} thread at {:p}>",
            object.get_type()?.lock().unwrap().name,
            &*object as *const KyaObject,
        )))
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a string",
            object.get_type()?.lock().unwrap().name
        )))
    }
}

pub fn thread_tp_new(
    ob_type: TypeRef,
    args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let target_arg = parse_arg(&args, 0, 1).or_else(|_| {
        Err(Error::RuntimeError(
            "Thread.new() expects a function as the first argument".to_string(),
        ))
    })?;

    Ok(KyaObject::from_thread_object(ThreadObject {
        ob_type: ob_type.clone(),
        target: target_arg.clone(),
        thread_handle: None,
    }))
}

pub fn thread_start(
    _callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    if args.len() != 0 {
        return Err(Error::RuntimeError(
            "Thread.start() takes no arguments".to_string(),
        ));
    }

    if receiver.is_none() {
        return Err(Error::RuntimeError(
            "Thread.start() must be called on an instance".to_string(),
        ));
    }

    let receiver = receiver.unwrap();

    if let KyaObject::ThreadObject(ref mut thread_obj) = *receiver.lock().unwrap() {
        let target = thread_obj.target.clone();

        let thread_handle = thread::spawn(move || {
            kya_acquire_lock();

            let result = kya_call(target.clone(), &mut vec![], None);

            if result.is_err() {
                eprintln!("{}", result.as_ref().err().unwrap());
            }

            kya_release_lock();

            result
        });

        thread_obj.thread_handle = Some(thread_handle);

        Ok(NONE_OBJECT.clone())
    } else {
        return Err(Error::RuntimeError(
            "The object is not a thread".to_string(),
        ));
    }
}

pub fn thread_join(
    _callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    if args.len() != 0 {
        return Err(Error::RuntimeError(
            "Thread.join() takes no arguments".to_string(),
        ));
    }

    if receiver.is_none() {
        return Err(Error::RuntimeError(
            "Thread.join() must be called on an instance".to_string(),
        ));
    }

    let receiver = receiver.unwrap();

    if let KyaObject::ThreadObject(ref mut thread_obj) = *receiver.lock().unwrap() {
        if let Some(handle) = thread_obj.thread_handle.take() {
            kya_release_lock();

            let _ = handle
                .join()
                .map_err(|_| Error::RuntimeError("Thread join failed".to_string()))?;

            kya_acquire_lock();

            Ok(NONE_OBJECT.clone())
        } else {
            Err(Error::RuntimeError(
                "Thread has not been started".to_string(),
            ))
        }
    } else {
        Err(Error::RuntimeError(
            "The object is not a thread".to_string(),
        ))
    }
}

pub fn thread_tp_init(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    Ok(NONE_OBJECT.clone())
}

pub static THREAD_OBJECT: Lazy<TypeRef> = Lazy::new(|| {
    let dict = Arc::new(Mutex::new(HashMap::new()));

    dict.lock()
        .unwrap()
        .insert("start".to_string(), rs_function_new(thread_start));

    dict.lock()
        .unwrap()
        .insert("join".to_string(), rs_function_new(thread_join));

    Type::as_ref(Type {
        ob_type: Some(BASE_TYPE.clone()),
        name: "threads.Thread".to_string(),
        tp_repr: Some(thread_tp_repr),
        tp_new: Some(thread_tp_new),
        tp_init: Some(thread_tp_init),
        dict: dict,
        ..Default::default()
    })
});
