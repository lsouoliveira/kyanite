use crate::objects::{Context, KyaError, KyaNone, KyaObject, KyaObjectKind, KyaString};

pub fn kya_print(context: &Context) -> Result<Box<dyn KyaObject>, KyaError> {
    println!("Hello from Kya!");

    Ok(Box::new(KyaNone {}))
}
