use crate::objects::{Context, KyaError, KyaNone, KyaObject, KyaObjectKind, KyaString};

pub fn kya_print(
    context: &Context,
    args: Vec<Box<dyn KyaObject>>,
) -> Result<Box<dyn KyaObject>, KyaError> {
    if args.is_empty() {
        return Err(KyaError::RuntimeError(
            "print() requires at least one argument".to_string(),
        ));
    }

    let mut output = String::new();

    for arg in args {
        if let Some(string) = arg.as_any().downcast_ref::<KyaString>() {
            output.push_str(&string.value);
        } else {
            return Err(KyaError::RuntimeError(
                "print() only accepts strings".to_string(),
            ));
        }
    }

    println!("{}", output);

    Ok(Box::new(KyaNone {}))
}
