use crate::errors::Error;
use crate::interpreter::{Interpreter, SOCKET_TYPE};
use crate::objects::base::KyaObjectRef;

pub fn kya_socket(
    interpreter: &mut Interpreter,
    _callable: KyaObjectRef,
    _args: Vec<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    interpreter.get_type(SOCKET_TYPE).borrow().new(
        interpreter,
        interpreter.get_type(SOCKET_TYPE),
        vec![],
    )
}
