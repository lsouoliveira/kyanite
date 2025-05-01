use crate::ast::ASTNode;
use crate::errors::Error;
use crate::interpreter::Interpreter;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum KyaObject {
    String(KyaString),
    Number(f64),
    RsFunction(KyaRsFunction),
    Function(KyaFunction),
    Frame(KyaFrame),
    Class(KyaClass),
    None(KyaNone),
    InstanceObject(KyaInstanceObject),
}

impl KyaObject {
    pub fn repr(&self) -> String {
        match self {
            KyaObject::String(s) => s.value.clone(),
            KyaObject::RsFunction(f) => f.name.clone(),
            KyaObject::None(_) => "None".to_string(),
            KyaObject::Number(n) => n.to_string(),
            KyaObject::Function(f) => format!("Function({:?})", f.name),
            KyaObject::Frame(f) => format!("Frame({:?})", f.locals),
            KyaObject::Class(c) => format!("Class({:?})", c.name),
            KyaObject::InstanceObject(i) => format!("InstanceObject({:?})", i.attributes),
        }
    }
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

#[derive(Debug, Clone, PartialEq)]
pub struct KyaFunction {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<Box<ASTNode>>,
}

impl KyaFunction {
    pub fn new(name: String, parameters: Vec<String>, body: Vec<Box<ASTNode>>) -> Self {
        KyaFunction {
            name,
            parameters,
            body,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KyaFrame {
    pub locals: Context,
}

impl KyaFrame {
    pub fn new() -> Self {
        KyaFrame {
            locals: Context::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KyaClass {
    pub name: String,
    pub body: Vec<Box<ASTNode>>,
}

impl KyaClass {
    pub fn new(name: String, body: Vec<Box<ASTNode>>) -> Self {
        KyaClass { name, body }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KyaInstanceObject {
    pub attributes: Context,
}

impl KyaInstanceObject {
    pub fn new(attributes: Context) -> Self {
        KyaInstanceObject { attributes }
    }

    pub fn get_attribute(&self, name: &str) -> Option<Rc<KyaObject>> {
        if let Some(object) = self.attributes.get(name) {
            Some(object.clone())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub objects: HashMap<String, Rc<KyaObject>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            objects: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Rc<KyaObject>> {
        if let Some(object) = self.objects.get(name) {
            Some(object.clone())
        } else {
            None
        }
    }

    pub fn register(&mut self, name: String, object: Rc<KyaObject>) {
        self.objects.insert(name, object);
    }

    pub fn keys(&self) -> Vec<String> {
        self.objects.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context() {
        let context = Context::new();
        assert_eq!(context.objects.len(), 0);
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
        let function = KyaRsFunction::new(String::from("test_function"), |_, _| {
            Ok(Rc::new(KyaObject::None(KyaNone)))
        });
        assert_eq!(function.name, "test_function");
    }

    #[test]
    fn test_kya_rs_function_call() {
        let function = KyaRsFunction::new(String::from("test_function"), |_, _| {
            Ok(Rc::new(KyaObject::None(KyaNone)))
        });
        let result = function.call(&Context::new(), vec![]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_kya_function() {
        let kya_function = KyaFunction::new(String::from("test_function"), vec![], vec![]);
        assert_eq!(kya_function.name, "test_function");
    }

    #[test]
    fn test_kya_frame() {
        let kya_frame = KyaFrame::new();
        assert_eq!(kya_frame.locals.objects.len(), 0);
    }
}
