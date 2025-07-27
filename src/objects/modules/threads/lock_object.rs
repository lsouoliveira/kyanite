use crate::errors::Error;
use crate::interpreter::NONE_OBJECT;
use crate::lock::{kya_acquire_lock, kya_release_lock};
use crate::objects::base::{
    kya_call, KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef, BASE_TYPE,
};
use crate::objects::rs_function_object::rs_function_new;
use crate::objects::string_object::string_new;
use crate::objects::utils::{parse_arg, parse_receiver};

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex};

pub struct LockObject {
    pub ob_type: TypeRef,
    pub lock: Mutex<bool>,
    pub cond: Condvar,
}

impl LockObject {
    pub fn acquire(&mut self) -> Result<(), Error> {
        let mut locked = self
            .lock
            .lock()
            .map_err(|_| Error::RuntimeError("Failed to acquire lock".to_string()))?;

        while *locked {
            locked = self.cond.wait(locked).map_err(|_| {
                Error::RuntimeError("Failed to wait on condition variable".to_string())
            })?;
        }

        *locked = true;

        Ok(())
    }

    pub fn release(&mut self) -> Result<(), Error> {
        let mut locked = self
            .lock
            .lock()
            .map_err(|_| Error::RuntimeError("Failed to acquire lock".to_string()))?;

        *locked = false;

        self.cond.notify_one();

        Ok(())
    }
}

impl KyaObjectTrait for LockObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn lock_new() -> Result<KyaObjectRef, Error> {
    Ok(KyaObject::from_lock_object(LockObject {
        ob_type: LOCK_TYPE.clone(),
        lock: Mutex::new(false),
        cond: Condvar::new(),
    }))
}

pub fn lock_tp_repr(
    callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let object = callable.lock().unwrap();

    if let KyaObject::LockObject(_) = &*object {
        Ok(string_new(&format!(
            "<{} lock at {:p}>",
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

pub fn lock_tp_new(
    _ob_type: TypeRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    lock_new()
}

pub fn lock_tp_init(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    Ok(NONE_OBJECT.clone())
}

pub fn lock_acquire(
    _callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let _ = parse_arg(args, 0, 0)?;
    let instance = parse_receiver(&receiver)?;

    if let KyaObject::LockObject(ref mut lock_object) = *instance.lock().unwrap() {
        kya_release_lock();
        lock_object.acquire()?;
        kya_acquire_lock();

        Ok(NONE_OBJECT.clone())
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a lock",
            instance.lock().unwrap().get_type()?.lock().unwrap().name
        )))
    }
}

pub fn lock_release(
    _callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let _ = parse_arg(args, 0, 0)?;
    let instance = parse_receiver(&receiver)?;

    if let KyaObject::LockObject(ref mut lock_object) = *instance.lock().unwrap() {
        kya_release_lock();
        lock_object.release()?;
        kya_acquire_lock();

        Ok(NONE_OBJECT.clone())
    } else {
        Err(Error::RuntimeError(format!(
            "The object '{}' is not a lock",
            instance.lock().unwrap().get_type()?.lock().unwrap().name
        )))
    }
}

pub static LOCK_TYPE: Lazy<TypeRef> = Lazy::new(|| {
    let dict = Arc::new(Mutex::new(HashMap::new()));

    dict.lock()
        .unwrap()
        .insert("acquire".to_string(), rs_function_new(lock_acquire));

    dict.lock()
        .unwrap()
        .insert("release".to_string(), rs_function_new(lock_release));

    Type::as_ref(Type {
        ob_type: Some(BASE_TYPE.clone()),
        name: "sockets.Lock".to_string(),
        tp_repr: Some(lock_tp_repr),
        tp_new: Some(lock_tp_new),
        tp_init: Some(lock_tp_init),
        dict: dict,
        ..Default::default()
    })
});
