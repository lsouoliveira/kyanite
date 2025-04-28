use std::collections::HashMap;

#[derive(Debug, Clone)]
enum KyaError {
    RuntimeError(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum KyaObjectKind {
    String,
    KyaRsFunction,
}

pub trait KyaObject {
    fn kind(&self) -> KyaObjectKind;
    fn as_any(&self) -> &dyn std::any::Any;
}

pub struct KyaString {
    pub value: String,
}

pub type KyaRsFunctionPtr = fn(&Context) -> Result<Box<dyn KyaObject>, KyaError>;
pub struct KyaRsFunction {
    pub name: String,
    pub function: KyaRsFunctionPtr,
}

pub struct Context {
    pub parent: Option<Box<Context>>,
    pub objects: HashMap<String, Box<dyn KyaObject>>,
}

impl std::fmt::Display for KyaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KyaError::RuntimeError(msg) => write!(f, "Runtime Error: {}", msg),
        }
    }
}

impl Context {
    pub fn new(parent: Option<Box<Context>>) -> Self {
        Context {
            parent,
            objects: HashMap::new(),
        }
    }
}

impl KyaObject for KyaString {
    fn kind(&self) -> KyaObjectKind {
        KyaObjectKind::String
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl KyaRsFunction {
    pub fn new(name: String, function: KyaRsFunctionPtr) -> Self {
        KyaRsFunction { name, function }
    }

    pub fn call(&self, context: &Context) -> Result<Box<dyn KyaObject>, KyaError> {
        (self.function)(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context() {
        let context = Context::new(None);
        assert_eq!(context.objects.len(), 0);
    }

    #[test]
    fn test_context_with_parent() {
        let parent_context = Context::new(None);
        let child_context = Context::new(Some(Box::new(parent_context)));
        assert_eq!(child_context.objects.len(), 0);
    }

    #[test]
    fn test_kya_string() {
        let kya_string = KyaString {
            value: String::from("Hello, Kya!"),
        };
        assert_eq!(kya_string.value, "Hello, Kya!");
        let object = Box::new(kya_string);
        assert_eq!(object.kind(), KyaObjectKind::String);
    }

    #[test]
    fn test_kya_rs_function() {
        let function = |_context: &Context| -> Result<Box<dyn KyaObject>, KyaError> {
            let result = KyaString {
                value: String::from("Hello from KyaRsFunction!"),
            };

            Ok(Box::new(result))
        };

        let kya_rs_function = KyaRsFunction::new(String::from("test_function"), function);
        let context = Context::new(None);
        let result = kya_rs_function.call(&context);
        assert!(result.is_ok());
        let result_object = result.unwrap();
        assert_eq!(result_object.kind(), KyaObjectKind::String);
        let string = result_object.as_any().downcast_ref::<KyaString>();
        assert!(string.is_some());
        assert_eq!(string.unwrap().value, "Hello from KyaRsFunction!");
    }
}
