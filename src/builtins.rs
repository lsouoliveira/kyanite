use crate::builtins_::string::kya_string_new;
use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::{kya_string_as_string, unpack_string, KyaNone, KyaObject};

use std::io::Write;
use std::rc::Rc;

pub fn kya_print(
    interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    if args.is_empty() {
        return Err(Error::TypeError(
            "print() requires at least one argument".to_string(),
        ));
    }

    let mut output = String::new();

    for arg in args {
        if let KyaObject::InstanceObject(_) = arg.as_ref() {
            let result = arg.get_attribute("__repr__").call(interpreter, vec![])?;
            let value = kya_string_as_string(&result)?;

            output.push_str(value.as_str());
        } else {
            output.push_str(&arg.repr());
        }
    }

    println!("{}", output);

    Ok(Rc::new(KyaObject::None(KyaNone {})))
}

pub fn kya_globals(
    interpreter: &mut Interpreter,
    _: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    for name in interpreter.context.keys() {
        println!("{}", name);
    }

    Ok(Rc::new(KyaObject::None(KyaNone {})))
}

pub fn kya_input(
    _interpreter: &mut Interpreter,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let arg = unpack_string(&args, 0, 1).unwrap_or_else(|_| kya_string_new("").unwrap());
    let prompt = kya_string_as_string(&arg)?;

    print!("{}", prompt);

    let _ = std::io::stdout().flush();
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .map_err(|_| Error::RuntimeError("Failed to read input".to_string()))?;

    Ok(kya_string_new(&input.trim_end().to_string()).unwrap())
}
