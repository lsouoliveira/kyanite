use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::errors::Error;
use crate::internal::socket::Connection;
use crate::interpreter::Interpreter;
use crate::objects::base::{KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef};
use crate::objects::bytes_object::BytesObject;
use crate::objects::rs_function_object::RsFunctionObject;
use crate::objects::utils::{number_object_to_float, parse_arg};

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

pub fn create_connection_type(
    _: &mut Interpreter,
    ob_type: TypeRef,
    rs_function_type: TypeRef,
) -> TypeRef {
    let dict = Rc::new(RefCell::new(HashMap::new()));

    dict.borrow_mut().insert(
        "recv".to_string(),
        KyaObject::from_rs_function_object(RsFunctionObject::new(
            rs_function_type.clone(),
            connection_read,
        )),
    );

    Type::as_ref(Type {
        ob_type: Some(ob_type.clone()),
        name: "sockets.Connection".to_string(),
        dict,
        ..Default::default()
    })
}

pub fn connection_read(
    interpreter: &mut Interpreter,
    callable: KyaObjectRef,
    args: Vec<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let arg = parse_arg(&args, 0, 1)?;
    let buffer_size = number_object_to_float(&arg)? as usize;
    let instance = interpreter.resolve_self()?;

    if let KyaObject::ConnectionObject(ref mut connection_obj) = *instance.borrow_mut() {
        let data = connection_obj.read(buffer_size)?;

        Ok(KyaObject::from_bytes_object(BytesObject {
            ob_type: interpreter.get_type("Bytes"),
            value: data,
        }))
    } else {
        Err(Error::RuntimeError(
            "Expected a Connection object".to_string(),
        ))
    }
}
