use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum KyaError {
    RuntimeError(String),
    UndefinedVariable(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum KyaObjectKind {
    String,
    RsFunction,
    None,
}

pub trait KyaObject {
    fn kind(&self) -> KyaObjectKind;
    fn as_any(&self) -> &dyn std::any::Any;
    fn dup(&self) -> Box<dyn KyaObject>;
}

pub struct KyaNone;

#[derive(Debug, Clone)]
pub struct KyaString {
    pub value: String,
}

pub type KyaRsFunctionPtr =
    fn(&Context, Vec<Box<dyn KyaObject>>) -> Result<Box<dyn KyaObject>, KyaError>;
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
            KyaError::UndefinedVariable(var) => write!(f, "Undefined Variable: {}", var),
        }
    }
}

impl std::fmt::Display for KyaObjectKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KyaObjectKind::String => write!(f, "String"),
            KyaObjectKind::RsFunction => write!(f, "RsFunction"),
            KyaObjectKind::None => write!(f, "None"),
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

    pub fn register(&mut self, name: String, object: Box<dyn KyaObject>) {
        self.objects.insert(name, object);
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn KyaObject>> {
        if let Some(object) = self.objects.get(name) {
            Some(object)
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            None
        }
    }
}

impl KyaObject for KyaNone {
    fn kind(&self) -> KyaObjectKind {
        KyaObjectKind::None
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn dup(&self) -> Box<dyn KyaObject> {
        Box::new(KyaNone {})
    }
}

impl KyaString {
    pub fn new(value: String) -> Self {
        KyaString { value }
    }

    pub fn dup(&self) -> Box<dyn KyaObject> {
        Box::new(KyaString {
            value: self.value.clone(),
        })
    }
}

impl KyaObject for KyaString {
    fn kind(&self) -> KyaObjectKind {
        KyaObjectKind::String
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn dup(&self) -> Box<dyn KyaObject> {
        Box::new(KyaString {
            value: self.value.clone(),
        })
    }
}

impl KyaRsFunction {
    pub fn new(name: String, function: KyaRsFunctionPtr) -> Self {
        KyaRsFunction { name, function }
    }

    pub fn call(
        &self,
        context: &Context,
        args: Vec<Box<dyn KyaObject>>,
    ) -> Result<Box<dyn KyaObject>, KyaError> {
        (self.function)(context, args)
    }

    pub fn dup(&self) -> Box<dyn KyaObject> {
        Box::new(KyaRsFunction {
            name: self.name.clone(),
            function: self.function,
        })
    }
}

impl KyaObject for KyaRsFunction {
    fn kind(&self) -> KyaObjectKind {
        KyaObjectKind::RsFunction
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn dup(&self) -> Box<dyn KyaObject> {
        Box::new(KyaRsFunction {
            name: self.name.clone(),
            function: self.function,
        })
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
        let function = |_context: &Context,
                        _args: Vec<Box<dyn KyaObject>>|
         -> Result<Box<dyn KyaObject>, KyaError> {
            let result = KyaString {
                value: String::from("Hello from KyaRsFunction!"),
            };

            Ok(Box::new(result))
        };

        let kya_rs_function = KyaRsFunction::new(String::from("test_function"), function);
        let context = Context::new(None);
        let result = kya_rs_function.call(&context, vec![]);
        assert!(result.is_ok());
        let result_object = result.unwrap();
        assert_eq!(result_object.kind(), KyaObjectKind::String);
        let string = result_object.as_any().downcast_ref::<KyaString>();
        assert!(string.is_some());
        assert_eq!(string.unwrap().value, "Hello from KyaRsFunction!");
    }
}
