use std::collections::HashMap;

use crate::errors::Error;
use crate::internal::socket::Connection;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef, BASE_TYPE};
use crate::objects::bytes_object::bytes_new;
use crate::objects::rs_function_object::rs_function_new;
use crate::objects::utils::{number_object_to_float, parse_arg, parse_receiver};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

pub struct ConnectionObject {
    ob_type: TypeRef,
    connection: Connection,
}

impl ConnectionObject {
    pub fn new(ob_type: TypeRef, connection: Connection) -> Self {
        Self {
            ob_type,
            connection,
        }
    }

    pub fn read(&mut self, buffer_size: usize) -> Result<Vec<u8>, Error> {
        self.connection.read(buffer_size).map_err(|e| {
            Error::RuntimeError(format!("Failed to read from connection: {}", e.to_string()))
        })
    }
}

impl KyaObjectTrait for ConnectionObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn connection_new(connection: Connection) -> KyaObjectRef {
    KyaObject::from_connection_object(ConnectionObject {
        ob_type: SOCKETS_CONNECTION_TYPE.clone(),
        connection,
    })
}

pub fn connection_read(
    _callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let instance = parse_receiver(&receiver)?;
    let arg = parse_arg(&args, 0, 1)?;
    let buffer_size = number_object_to_float(&arg)? as usize;

    if let KyaObject::ConnectionObject(ref mut connection_obj) = *instance.lock().unwrap() {
        let data = connection_obj.read(buffer_size)?;

        Ok(bytes_new(data))
    } else {
        Err(Error::RuntimeError(
            "Expected a Connection object".to_string(),
        ))
    }
}

pub static SOCKETS_CONNECTION_TYPE: Lazy<TypeRef> = Lazy::new(|| {
    let dict = Arc::new(Mutex::new(HashMap::new()));

    dict.lock()
        .unwrap()
        .insert("recv".to_string(), rs_function_new(connection_read));

    Type::as_ref(Type {
        ob_type: Some(BASE_TYPE.clone()),
        name: "sockets.Connection".to_string(),
        dict,
        ..Default::default()
    })
});
