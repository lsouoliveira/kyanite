use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::{Context, KyaNone, KyaObject, KyaSocket};

use std::cell::RefCell;
use std::rc::Rc;

pub fn kya_socket_init(
    _interpreter: &mut Interpreter,
    _args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    Ok(Rc::new(KyaObject::None(KyaNone {})))
}

pub fn kya_socket_new() -> Result<Rc<KyaObject>, Error> {
    let instance = Rc::new(KyaObject::Socket(KyaSocket::new(
        None,
        RefCell::new(Context::new()),
    )));

    Ok(instance)
}
