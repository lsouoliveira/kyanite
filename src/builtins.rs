use crate::errors::Error;
use crate::objects::{Context, KyaNone, KyaObject};
use std::rc::Rc;

pub fn kya_print(_: &Context, args: Vec<Rc<KyaObject>>) -> Result<Rc<KyaObject>, Error> {
    if args.is_empty() {
        return Err(Error::RuntimeError(
            "print() requires at least one argument".to_string(),
        ));
    }

    let mut output = String::new();

    for arg in args {
        if let KyaObject::String(ref s) = *arg {
            output.push_str(&s.value);
        } else {
            return Err(Error::RuntimeError(
                "print() only accepts string arguments".to_string(),
            ));
        }
    }

    println!("{}", output);

    Ok(Rc::new(KyaObject::None(KyaNone {})))
}
