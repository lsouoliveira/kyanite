use crate::errors::Error;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum KyaObject {
    String(KyaString),
    RsFunction(KyaRsFunction),
    None(KyaNone),
}

#[derive(Debug, Clone, PartialEq)]
pub struct KyaNone;

#[derive(Debug, Clone, PartialEq)]
pub struct KyaString {
    pub value: String,
}

pub type KyaRsFunctionPtr = fn(&Context, Vec<Rc<KyaObject>>) -> Result<Rc<KyaObject>, Error>;

#[derive(Debug, Clone, PartialEq)]
pub struct KyaRsFunction {
    pub name: String,
    pub function: KyaRsFunctionPtr,
}
impl KyaRsFunction {
    pub fn new(name: String, function: KyaRsFunctionPtr) -> Self {
        KyaRsFunction { name, function }
    }

    pub fn call(
        &self,
        context: &Context,
        args: Vec<Rc<KyaObject>>,
    ) -> Result<Rc<KyaObject>, Error> {
        (self.function)(context, args)
    }
}

pub struct Context {
    pub parent: Option<Box<Context>>,
    pub objects: HashMap<String, Rc<KyaObject>>,
}

impl Context {
    pub fn new(parent: Option<Box<Context>>) -> Self {
        Context {
            parent,
            objects: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Rc<KyaObject>> {
        if let Some(object) = self.objects.get(name) {
            Some(object.clone())
        } else if let Some(ref parent) = self.parent {
            parent.get(name)
        } else {
            None
        }
    }

    pub fn register(&mut self, name: String, object: Rc<KyaObject>) {
        self.objects.insert(name, object);
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
    }

    #[test]
    fn test_kya_rs_function() {
        let function = KyaRsFunction::new(
            String::from("test_function"),
            |_, _| Ok(Rc::new(KyaObject::None(KyaNone))),
        );
        assert_eq!(function.name, "test_function");
    }

    #[test]
    fn test_kya_rs_function_call() {
        let function = KyaRsFunction::new(
            String::from("test_function"),
            |_, _| Ok(Rc::new(KyaObject::None(KyaNone))),
        );
        let result = function.call(&Context::new(None), vec![]);
        assert!(result.is_ok());
    }
}
