use crate::errors::Error;
use crate::objects::base::{kya_call, KyaObjectRef};
use crate::objects::class_object::class_new;
use crate::objects::modules::sockets::socket_object::SOCKET_TYPE;

pub fn kya_socket(
    _callable: KyaObjectRef,
    _args: &mut Vec<KyaObjectRef>,
    _receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let socket_class = class_new(SOCKET_TYPE.clone());

    kya_call(socket_class, &mut vec![], None)
}
