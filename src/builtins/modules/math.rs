use crate::builtins::number::kya_number_new;
use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::{unpack_args, Context, KyaModule, KyaObject, KyaRsFunction};

use std::cell::RefCell;
use std::rc::Rc;

pub fn sqrt(
    _interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let arg = unpack_args(&args, 0, 1).unwrap();

    Ok(kya_number_new(arg.as_number()?.sqrt())?)
}

pub fn abs(
    _interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let arg = unpack_args(&args, 0, 1).unwrap();

    Ok(kya_number_new(arg.as_number()?.abs())?)
}

pub fn pack_module() -> KyaObject {
    let mut objects = Context::new();

    objects.register(
        "sqrt".to_string(),
        Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
            "sqrt".to_string(),
            sqrt,
        ))),
    );

    objects.register(
        "abs".to_string(),
        Rc::new(KyaObject::RsFunction(KyaRsFunction::new(
            "abs".to_string(),
            abs,
        ))),
    );

    KyaObject::Module(KyaModule {
        name: "math".to_string(),
        objects: RefCell::new(objects),
    })
}
