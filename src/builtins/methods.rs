use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::base::KyaObjectRef;
use crate::objects::utils::string_object_to_string;

pub fn kya_print(
    interpreter: &mut Interpreter,
    _callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let mut output = String::new();
    for arg in args {
        let repr = arg.borrow().get_type()?.borrow().repr(
            interpreter,
            arg.clone(),
            &mut vec![],
            receiver.clone(),
        )?;
        output.push_str(&string_object_to_string(&repr)?);
    }

    println!("{}", output);

    Ok(interpreter.get_none())
}
