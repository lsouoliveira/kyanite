use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::base::{KyaObject, KyaObjectRef};
use crate::objects::class_object::ClassObject;
use crate::objects::modules::sockets::SOCKET_TYPE;

pub fn kya_socket(
    interpreter: &mut Interpreter,
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let socket_object = KyaObject::from_class_object(ClassObject {
        ob_type: interpreter.get_type(SOCKET_TYPE),
    });

    socket_object.lock().unwrap().get_type()?.lock().unwrap().call(
        interpreter,
        socket_object.clone(),
        &mut vec![],
        receiver.clone(),
    )
}
