use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::errors::Error;
use crate::internal::socket::Connection;
use crate::interpreter::Interpreter;
use crate::objects::base::{KyaObjectRef, KyaObjectTrait, Type, TypeRef};

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
}

impl KyaObjectTrait for ConnectionObject {
    fn get_type(&self) -> TypeRef {
        self.ob_type.clone()
    }
}

pub fn create_connection_type(
    _: &mut Interpreter,
    ob_type: TypeRef,
    _rs_function_type: TypeRef,
) -> TypeRef {
    let dict = Rc::new(RefCell::new(HashMap::new()));

    // dict.borrow_mut().insert(
    //     "bind".to_string(),
    //     KyaObject::from_rs_function_object(RsFunctionObject::new(
    //         rs_function_type.clone(),
    //         socket_bind,
    //     )),
    // );

    Type::as_ref(Type {
        ob_type: Some(ob_type.clone()),
        name: "sockets.Connection".to_string(),
        // tp_new: Some(socket_new),
        // tp_init: Some(socket_tp_init),
        dict,
        ..Default::default()
    })
}
