use crate::errors::Error;
use crate::objects::{Context, KyaNone, KyaObject};
use std::rc::Rc;

pub fn kya_print(_: &Context, args: Vec<Rc<KyaObject>>) -> Result<Rc<KyaObject>, Error> {
    if args.is_empty() {
        return Err(Error::TypeError(
            "print() requires at least one argument".to_string(),
        ));
    }

    let mut output = String::new();

    for arg in args {
        output.push_str(arg.repr().as_str());
    }

    println!("{}", output);

    Ok(Rc::new(KyaObject::None(KyaNone {})))
}

pub fn kya_globals(context: &Context, _: Vec<Rc<KyaObject>>) -> Result<Rc<KyaObject>, Error> {
    for name in context.keys() {
        println!("{}", name);
    }

    Ok(Rc::new(KyaObject::None(KyaNone {})))
}
