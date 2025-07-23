use crate::errors::Error;
use crate::objects::base::KyaObjectRef;
use crate::objects::none_object::none_new;
use crate::objects::utils::string_object_to_string;

pub fn kya_print(
    _callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error> {
    let mut output = String::new();

    for arg in args {
        let arg_type = arg.lock().unwrap().get_type()?;
        let tp_repr = arg_type.lock().unwrap().tp_repr;

        if let Some(repr_fn) = tp_repr {
            let repr = repr_fn(arg.clone(), &mut vec![], receiver.clone())?;

            output.push_str(&string_object_to_string(&repr)?);
        } else {
            return Err(Error::RuntimeError(format!(
                "Type '{}' does not have a tp_repr method",
                arg_type.lock().unwrap().name
            )));
        }
    }

    println!("{}", output);

    Ok(none_new()?)
}
