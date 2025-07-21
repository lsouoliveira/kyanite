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

        let repr = arg_type
            .lock()
            .unwrap()
            .repr(arg.clone(), &mut vec![], receiver.clone())?;

        output.push_str(&string_object_to_string(&repr)?);
    }

    println!("{}", output);

    Ok(none_new()?)
}
