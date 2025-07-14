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
    Method(KyaMethod),
    RsMethod(KyaRsMethod),
    Bool(bool),
    Module(KyaModule),
    List(KyaList),
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
            KyaObject::Method(m) => format!("Method({:?})", m.function),
            KyaObject::RsMethod(m) => format!("RsMethod({:?})", m.function),
            KyaObject::Bool(b) => b.to_string(),
            KyaObject::Module(m) => format!("Module({:?})", m.name),
            KyaObject::List(l) => {
                let items: Vec<String> = l.items.iter().map(|item| item.repr()).collect();
                format!("List([{}])", items.join(", "))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KyaNone;

#[derive(Debug, Clone, PartialEq)]
pub struct KyaString {
    pub value: String,
}

impl KyaString {
    pub fn new(value: String) -> Self {
        KyaString { value }
    }
}

pub type KyaRsFunctionPtr =
    fn(&mut Interpreter, Vec<Rc<KyaObject>>) -> Result<Rc<KyaObject>, Error>;

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
        interpreter: &mut Interpreter,
        args: Vec<Rc<KyaObject>>,
    ) -> Result<Rc<KyaObject>, Error> {
        (self.function)(interpreter, args)
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
    name: String,
    pub attributes: Context,
}

impl KyaInstanceObject {
    pub fn new(name: String, attributes: Context) -> Self {
        KyaInstanceObject { name, attributes }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_attribute(&self, name: &str) -> Option<Rc<KyaObject>> {
        if let Some(object) = self.attributes.get(name) {
            Some(object.clone())
        } else {
            None
        }
    }

    pub fn get_string_attribute(&self, name: &str) -> Option<String> {
        if let Some(object) = self.get_attribute(name) {
            if let KyaObject::String(s) = object.as_ref() {
                return Some(s.value.clone());
            }
        }
        None
    }

    pub fn get_number_attribute(&self, name: &str) -> Option<f64> {
        if let Some(object) = self.get_attribute(name) {
            if let KyaObject::Number(n) = object.as_ref() {
                return Some(*n);
            }
        }
        None
    }

    pub fn get_bool_attribute(&self, name: &str) -> Option<bool> {
        if let Some(object) = self.get_attribute(name) {
            if let KyaObject::Bool(b) = object.as_ref() {
                return Some(*b);
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KyaMethod {
    pub function: Rc<KyaObject>,
    pub instance: Rc<KyaObject>,
}

impl KyaMethod {}

#[derive(Debug, Clone, PartialEq)]
pub struct KyaRsMethod {
    pub function: Rc<KyaObject>,
    pub instance: Rc<KyaObject>,
}

impl KyaRsMethod {}

#[derive(Debug, Clone, PartialEq)]
pub struct KyaModule {
    pub name: String,
    pub objects: Context,
}

impl KyaModule {
    pub fn new(name: String, objects: Context) -> Self {
        KyaModule { name, objects }
    }

    pub fn resolve(&self, name: &str) -> Option<Rc<KyaObject>> {
        self.objects.get(name)
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

#[derive(Debug, Clone, PartialEq)]
pub struct KyaList {
    items: Vec<Rc<KyaObject>>,
}

impl KyaList {
    pub fn new(items: Vec<Rc<KyaObject>>) -> Self {
        KyaList { items }
    }

    pub fn get(&self, index: usize) -> Option<&Rc<KyaObject>> {
        self.items.get(index)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

pub fn unpack_number(
    args: &[Rc<KyaObject>],
    index: usize,
    args_count: usize,
) -> Result<Rc<KyaObject>, Error> {
    if index >= args_count {
        return Err(Error::TypeError(format!(
            "Expected at least {} arguments, but got {}",
            index + 1,
            args_count
        )));
    }

    if let Some(arg) = args.get(index) {
        if let KyaObject::InstanceObject(obj) = arg.as_ref() {
            if obj.name() == "Number" {
                return Ok(arg.clone());
            }
        }
    }

    Err(Error::TypeError(format!(
        "Expected a Number at index {}, found {:?}",
        index, args[index]
    )))
}

pub fn unpack_string(
    args: &[Rc<KyaObject>],
    index: usize,
    args_count: usize,
) -> Result<Rc<KyaObject>, Error> {
    if index >= args_count {
        return Err(Error::TypeError(format!(
            "Expected at least {} arguments, but got {}",
            index + 1,
            args_count
        )));
    }

    if let Some(arg) = args.get(index) {
        if let KyaObject::InstanceObject(obj) = arg.as_ref() {
            if obj.name() == "String" {
                return Ok(arg.clone());
            }
        }
    }

    Err(Error::TypeError(format!(
        "Expected a String at index {}, found {:?}",
        index, args[index]
    )))
}

pub fn kya_number_as_float(object: &KyaObject) -> Result<f64, Error> {
    if let KyaObject::InstanceObject(obj) = object {
        if obj.name() == "Number" {
            return Ok(obj.get_number_attribute("__value__").unwrap());
        }
    };

    return Err(Error::TypeError("Expected a Number instance".to_string()));
}

pub fn kya_string_as_string(object: &KyaObject) -> Result<String, Error> {
    if let KyaObject::InstanceObject(obj) = object {
        if obj.name() == "String" {
            return Ok(obj.get_string_attribute("__value__").unwrap());
        }
    };

    return Err(Error::TypeError("Expected a String instance".to_string()));
}

pub fn kya_list_as_vec(object: &KyaObject) -> Result<Vec<Rc<KyaObject>>, Error> {
    if let KyaObject::InstanceObject(obj) = object {
        if obj.name() == "List" {
            if let Some(items) = obj.get_attribute("__items__") {
                if let KyaObject::List(list) = items.as_ref() {
                    return Ok(list.items.clone());
                }
            }
        }
    }

    Err(Error::TypeError("Expected a List instance".to_string()))
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
        let mut interpreter = Interpreter::new("".to_string());
        let result = function.call(&mut interpreter, vec![]);
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

    #[test]
    fn test_boolean_object() {
        let kya_bool = KyaObject::Bool(true);
        if let KyaObject::Bool(value) = kya_bool {
            assert!(value);
        } else {
            panic!("Expected a boolean object");
        }
    }
}
