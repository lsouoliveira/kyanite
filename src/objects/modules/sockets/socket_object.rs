use std::collections::HashMap;

use crate::errors::Error;
use crate::internal::socket::Connection;
use crate::internal::socket::{self};
use crate::interpreter::NONE_OBJECT;
use crate::objects::base::{
    kya_repr, KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef, BASE_TYPE,
};
use crate::objects::modules::sockets::connection_object::connection_new;
use crate::objects::number_object::NUMBER_TYPE;
use crate::objects::rs_function_object::rs_function_new;
use crate::objects::string_object::STRING_TYPE;
use crate::objects::utils::{
    number_object_to_float, parse_arg, parse_receiver, string_object_to_string,
};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

pub struct SocketObject {
    ob_type: TypeRef,
    socket: socket::Socket,
}

impl SocketObject {
    pub fn new(ob_type: TypeRef, socket: socket::Socket) -> Self {
        Self { ob_type, socket }
    }

    pub fn bind(&mut self, host: &str, port: u16) -> Result<(), Error> {
        self.socket.bind(host, port).map_err(|e| {
            Error::RuntimeError(format!(
                "Failed to bind socket to {}:{}. Error: {}",
                host, port, e
            ))
        })
    }

    pub fn accept(&mut self) -> Result<Connection, Error> {
        self.socket
            .accept()
            .map_err(|e| Error::RuntimeError(format!("Failed to accept connection. Error: {}", e)))
    }
}

impl KyaObjectTrait for SocketObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn socket_tp_new(
    ob_type: TypeRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let socket = socket::create_socket();

    Ok(KyaObject::from_socket_object(SocketObject::new(
        ob_type, socket,
    )))
}

pub fn socket_tp_init(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    Ok(NONE_OBJECT.clone())
}

pub fn socket_bind(
    _callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let instance = parse_receiver(&receiver)?;

    if let KyaObject::SocketObject(ref mut socket_object) = *instance.lock().unwrap() {
        let host = parse_arg(&args, 0, 2)?;
        let port = parse_arg(&args, 1, 2)?;

        host.lock()
            .unwrap()
            .is_instance_of(&STRING_TYPE)?
            .then_some(())
            .ok_or_else(|| Error::ValueError("The 'host' argument must be a string".to_string()))?;

        port.lock()
            .unwrap()
            .is_instance_of(&NUMBER_TYPE)?
            .then_some(())
            .ok_or_else(|| Error::ValueError("The 'port' argument must be a string".to_string()))?;

        socket_object.bind(
            &string_object_to_string(&host)?,
            number_object_to_float(&port)? as u16,
        )?;

        Ok(NONE_OBJECT.clone())
    } else {
        Err(Error::TypeError("Expected a Socket object".to_string()))
    }
}

pub fn socket_accept(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let instance = parse_receiver(&receiver)?;

    if let KyaObject::SocketObject(ref mut socket_object) = *instance.lock().unwrap() {
        let connection = socket_object.accept()?;

        Ok(connection_new(connection))
    } else {
        Err(Error::TypeError("Expected a Socket object".to_string()))
    }
}

pub static SOCKET_TYPE: Lazy<TypeRef> = Lazy::new(|| {
    let dict = Arc::new(Mutex::new(HashMap::new()));

    dict.lock()
        .unwrap()
        .insert("bind".to_string(), rs_function_new(socket_bind));

    dict.lock()
        .unwrap()
        .insert("accept".to_string(), rs_function_new(socket_accept));

    Type::as_ref(Type {
        ob_type: Some(BASE_TYPE.clone()),
        name: "sockets.Socket".to_string(),
        tp_new: Some(socket_tp_new),
        tp_init: Some(socket_tp_init),
        dict,
        ..Default::default()
    })
});
