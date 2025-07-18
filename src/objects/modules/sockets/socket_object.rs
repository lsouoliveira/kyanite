use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::errors::Error;
use crate::internal::socket;
use crate::interpreter::{Interpreter, NUMBER_TYPE, RS_FUNCTION_TYPE, STRING_TYPE};
use crate::objects::base::{
    generic_get_attr, KyaObject, KyaObjectRef, KyaObjectTrait, Type, TypeRef,
};
use crate::objects::rs_function_object::RsFunctionObject;
use crate::objects::utils::{number_object_to_float, parse_arg, string_object_to_string};

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
}

impl KyaObjectTrait for SocketObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn create_socket_type(interpreter: &mut Interpreter, ob_type: TypeRef) -> TypeRef {
    let dict = Rc::new(RefCell::new(HashMap::new()));
    let rs_function_type = interpreter.get_type(RS_FUNCTION_TYPE);

    dict.borrow_mut().insert(
        "bind".to_string(),
        KyaObject::from_rs_function_object(RsFunctionObject::new(
            rs_function_type.clone(),
            socket_bind,
        )),
    );

    Type::as_ref(Type {
        ob_type: Some(ob_type.clone()),
        name: "Socket".to_string(),
        tp_new: Some(socket_new),
        tp_get_attr: Some(generic_get_attr),
        dict,
        ..Default::default()
    })
}

pub fn socket_new(
    _interpreter: &mut Interpreter,
    ob_type: TypeRef,
    _args: Vec<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let socket = socket::create_socket();

    Ok(KyaObject::from_socket_object(SocketObject::new(
        ob_type, socket,
    )))
}

pub fn socket_bind(
    interpreter: &mut Interpreter,
    _callable: KyaObjectRef,
    args: Vec<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let instance = interpreter.resolve_self()?;

    if let KyaObject::SocketObject(ref mut socket_object) = *instance.borrow_mut() {
        let host = parse_arg(&args, 0, 2)?;
        let port = parse_arg(&args, 1, 2)?;

        host.borrow()
            .is_instance_of(&interpreter.get_type(STRING_TYPE))?
            .then_some(())
            .ok_or_else(|| Error::ValueError("The 'host' argument must be a string".to_string()))?;

        port.borrow()
            .is_instance_of(&interpreter.get_type(NUMBER_TYPE))?
            .then_some(())
            .ok_or_else(|| Error::ValueError("The 'port' argument must be a string".to_string()))?;

        socket_object.bind(
            &string_object_to_string(&host)?,
            number_object_to_float(&port)? as u16,
        )?;

        Ok(interpreter.get_none())
    } else {
        Err(Error::TypeError("Expected a Socket object".to_string()))
    }
}
