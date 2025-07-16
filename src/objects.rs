use crate::ast::ASTNode;
use crate::errors::Error;
use crate::internal::socket::Socket;
use crate::interpreter::Interpreter;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum KyaObject {
    String(KyaString),
    Number(KyaNumber),
    RsFunction(KyaRsFunction),
    Function(KyaFunction),
    Class(KyaClass),
    RsClass(KyaRsClass),
    None(KyaNone),
    InstanceObject(KyaInstanceObject),
    Method(KyaMethod),
    Bool(KyaBool),
    Module(KyaModule),
    List(KyaList),
    Socket(KyaSocket),
}

impl KyaObject {
    pub fn repr(&self) -> String {
        match self {
            KyaObject::String(s) => format!("String({})", s.value.borrow()),
            KyaObject::RsFunction(f) => f.name.clone(),
            KyaObject::None(_) => "None".to_string(),
            KyaObject::Number(n) => format!("Number({})", n.value.borrow()),
            KyaObject::Function(f) => format!("Function({:?})", f.name),
            KyaObject::Class(c) => format!("Class({:?})", c.name),
            KyaObject::InstanceObject(i) => format!("InstanceObject({:?})", i.name),
            KyaObject::Method(m) => format!("Method({:?})", m.function),
            KyaObject::Bool(b) => b.value.borrow().to_string(),
            KyaObject::Module(m) => format!("Module({})", m.name),
            KyaObject::List(l) => {
                let items: Vec<String> = l.items.borrow().iter().map(|item| item.repr()).collect();
                format!("List([{}])", items.join(", "))
            }
            KyaObject::RsClass(c) => format!("RsClass({:?})", c.name),
            KyaObject::Socket(_) => "Socket".to_string(),
        }
    }

    pub fn as_callable(&self) -> Option<&dyn Callable> {
        match self {
            KyaObject::RsFunction(f) => Some(f),
            KyaObject::Function(f) => Some(f),
            KyaObject::Class(c) => Some(c),
            KyaObject::RsClass(c) => Some(c),
            KyaObject::Method(m) => Some(m),
            _ => None,
        }
    }

    pub fn as_object_type(&self) -> Option<&dyn ObjectType> {
        match self {
            KyaObject::InstanceObject(i) => Some(i),
            KyaObject::Socket(s) => Some(s),
            KyaObject::Number(n) => Some(n),
            KyaObject::String(s) => Some(s),
            KyaObject::Bool(b) => Some(b),
            KyaObject::List(l) => Some(l),
            KyaObject::Module(m) => Some(m),
            _ => None,
        }
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Rc<KyaObject>>,
    ) -> Result<Rc<KyaObject>, Error> {
        if let Some(callable) = self.as_callable() {
            return callable.call(interpreter, args);
        }

        Err(Error::TypeError(format!(
            "Object of type {} is not callable",
            self.repr()
        )))
    }

    pub fn get_attribute(&self, name: &str) -> Result<Rc<KyaObject>, Error> {
        if let Some(getter) = self.as_object_type() {
            return getter.get_attribute(name);
        }

        Err(Error::RuntimeError(format!(
            "Object of type {} has no attribute '{}'",
            self.repr(),
            name
        )))
    }

    pub fn set_attribute(&self, name: String, value: Rc<KyaObject>) {
        if let Some(setter) = self.as_object_type() {
            setter.set_attribute(name, value);
        }
    }

    pub fn name(&self) -> String {
        if let Some(object_type) = self.as_object_type() {
            return object_type.name();
        }

        "Object".to_string()
    }

    pub fn assign(&self, value: Rc<KyaObject>) -> Result<Rc<KyaObject>, Error> {
        if let Some(object_type) = self.as_object_type() {
            return object_type.assign(value);
        }

        Err(Error::TypeError(format!(
            "Cannot assign to object of type {}",
            self.name()
        )))
    }

    pub fn as_bool(&self) -> Result<bool, Error> {
        if let KyaObject::Bool(b) = self {
            return Ok(b.value.borrow().clone());
        }

        Err(Error::TypeError(format!(
            "Expected a boolean, found {}",
            self.repr()
        )))
    }

    pub fn as_string(&self) -> Result<String, Error> {
        if let KyaObject::String(s) = self {
            return Ok(s.value.borrow().clone());
        }

        Err(Error::TypeError(format!(
            "Expected a string, found {}",
            self.repr()
        )))
    }

    pub fn as_number(&self) -> Result<f64, Error> {
        if let KyaObject::Number(n) = self {
            return Ok(n.value.borrow().clone());
        }

        Err(Error::TypeError(format!(
            "Expected a number, found {}",
            self.repr()
        )))
    }

    pub fn as_vector(&self) -> Result<Vec<Rc<KyaObject>>, Error> {
        if let KyaObject::List(l) = self {
            return Ok(l.items.borrow().clone());
        }

        Err(Error::TypeError(format!(
            "Expected a list, found {}",
            self.repr()
        )))
    }
}

pub trait Callable {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Rc<KyaObject>>,
    ) -> Result<Rc<KyaObject>, Error>;
}

pub trait ObjectType {
    fn get_attribute(&self, name: &str) -> Result<Rc<KyaObject>, Error>;
    fn set_attribute(&self, name: String, value: Rc<KyaObject>);
    fn name(&self) -> String;
    fn assign(&self, value: Rc<KyaObject>) -> Result<Rc<KyaObject>, Error>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct KyaNone;

#[derive(Debug, Clone, PartialEq)]
pub struct KyaString {
    pub value: RefCell<String>,
    pub instance: Rc<KyaObject>,
}

impl ObjectType for KyaString {
    fn get_attribute(&self, name: &str) -> Result<Rc<KyaObject>, Error> {
        self.instance.get_attribute(name)
    }

    fn set_attribute(&self, name: String, value: Rc<KyaObject>) {
        self.instance.set_attribute(name, value);
    }

    fn name(&self) -> String {
        "String".to_string()
    }

    fn assign(&self, value: Rc<KyaObject>) -> Result<Rc<KyaObject>, Error> {
        if let KyaObject::String(_) = value.as_ref() {
            self.value.replace(value.as_string()?.to_string());

            Ok(value)
        } else {
            Err(Error::TypeError(format!(
                "Cannot assign {} to attribute of type String",
                value.repr()
            )))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KyaNumber {
    pub value: RefCell<f64>,
    pub instance: Rc<KyaObject>,
}

impl ObjectType for KyaNumber {
    fn get_attribute(&self, name: &str) -> Result<Rc<KyaObject>, Error> {
        self.instance.get_attribute(name)
    }

    fn set_attribute(&self, name: String, value: Rc<KyaObject>) {
        self.instance.set_attribute(name, value);
    }

    fn name(&self) -> String {
        "Number".to_string()
    }

    fn assign(&self, value: Rc<KyaObject>) -> Result<Rc<KyaObject>, Error> {
        if let KyaObject::Number(_) = value.as_ref() {
            self.value.replace(value.as_number()?);

            Ok(value)
        } else {
            Err(Error::TypeError(format!(
                "Cannot assign {} to attribute of type Number",
                value.repr(),
            )))
        }
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
}

impl Callable for KyaRsFunction {
    fn call(
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

impl Callable for KyaFunction {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Rc<KyaObject>>,
    ) -> Result<Rc<KyaObject>, Error> {
        let frame = Rc::new(RefCell::new(KyaFrame::new()));

        if self.parameters.len() != args.len() {
            return Err(Error::RuntimeError(format!(
                "{}() takes {} argument(s) but {} were given",
                self.name,
                self.parameters.len(),
                args.len()
            )));
        }

        for (i, param) in self.parameters.iter().enumerate() {
            let arg = args[i].clone();
            frame.borrow_mut().locals.register(param.clone(), arg);
        }

        interpreter.frames.push(frame);

        let mut result = Rc::new(KyaObject::None(KyaNone {}));

        for stmt in &self.body {
            result = stmt.eval(interpreter)?;
        }

        interpreter.frames.pop();

        return Ok(result);
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

impl std::fmt::Display for KyaFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "KyaFrame(locals: {:?})",
            self.locals.objects.borrow().keys()
        )
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

impl Callable for KyaClass {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Rc<KyaObject>>,
    ) -> Result<Rc<KyaObject>, Error> {
        let frame = Rc::new(RefCell::new(KyaFrame::new()));

        interpreter.frames.push(frame.clone());

        self.body.iter().for_each(|stmt| {
            stmt.eval(interpreter).unwrap();
        });

        interpreter.frames.pop();

        let instance = Rc::new(KyaObject::InstanceObject(KyaInstanceObject::new(
            self.name.clone(),
            RefCell::new(frame.borrow().locals.clone()),
        )));

        call_constructor(interpreter, instance.clone(), args)?;

        Ok(instance)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KyaRsClass {
    pub name: String,
    pub parameters: Vec<String>,
    pub init_function: KyaRsFunctionPtr,
}

impl KyaRsClass {
    pub fn new(name: String, parameters: Vec<String>, init_function: KyaRsFunctionPtr) -> Self {
        KyaRsClass {
            name,
            parameters,
            init_function,
        }
    }

    pub fn instantiate(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Rc<KyaObject>>,
    ) -> Result<Rc<KyaObject>, Error> {
        (self.init_function)(interpreter, args)
    }
}

impl Callable for KyaRsClass {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Rc<KyaObject>>,
    ) -> Result<Rc<KyaObject>, Error> {
        let instance = self.instantiate(interpreter, args.clone())?;

        call_constructor(interpreter, instance.clone(), args)?;

        return Ok(instance);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KyaInstanceObject {
    name: String,
    pub attributes: RefCell<Context>,
}

impl ObjectType for KyaInstanceObject {
    fn get_attribute(&self, name: &str) -> Result<Rc<KyaObject>, Error> {
        if let Some(object) = self.attributes.borrow().get(name) {
            return Ok(object.clone());
        }

        Err(Error::RuntimeError(format!(
            "Instance of {} has no attribute '{}'",
            self.name, name
        )))
    }

    fn set_attribute(&self, name: String, value: Rc<KyaObject>) {
        self.attributes.borrow_mut().register(name, value);
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn assign(&self, value: Rc<KyaObject>) -> Result<Rc<KyaObject>, Error> {
        Err(Error::TypeError(format!(
            "Cannot assign {} to instance attribute of type {}",
            value.repr(),
            self.name
        )))
    }
}

impl KyaInstanceObject {
    pub fn new(name: String, attributes: RefCell<Context>) -> Self {
        KyaInstanceObject { name, attributes }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KyaMethod {
    pub function: Rc<KyaObject>,
    pub instance: Rc<KyaObject>,
}

impl KyaMethod {
    pub fn new(function: Rc<KyaObject>, instance: Rc<KyaObject>) -> Self {
        KyaMethod { function, instance }
    }
}

impl Callable for KyaMethod {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Rc<KyaObject>>,
    ) -> Result<Rc<KyaObject>, Error> {
        let frame = Rc::new(RefCell::new(KyaFrame::new()));

        frame
            .borrow_mut()
            .locals
            .register(String::from("self"), self.instance.clone());

        interpreter.frames.push(frame);

        let result = self.function.call(interpreter, args)?;

        interpreter.frames.pop();

        return Ok(result);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KyaBool {
    pub value: RefCell<bool>,
    pub instance: Rc<KyaObject>,
}

impl ObjectType for KyaBool {
    fn get_attribute(&self, name: &str) -> Result<Rc<KyaObject>, Error> {
        self.instance.get_attribute(name)
    }

    fn set_attribute(&self, name: String, value: Rc<KyaObject>) {
        self.instance.set_attribute(name, value);
    }

    fn name(&self) -> String {
        "Bool".to_string()
    }

    fn assign(&self, value: Rc<KyaObject>) -> Result<Rc<KyaObject>, Error> {
        if let KyaObject::Bool(_) = value.as_ref() {
            self.value.replace(value.as_bool()?);

            Ok(value)
        } else {
            Err(Error::TypeError(format!(
                "Cannot assign {} to attribute of type Bool",
                value.repr(),
            )))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KyaModule {
    pub name: String,
    pub objects: RefCell<Context>,
}

impl ObjectType for KyaModule {
    fn get_attribute(&self, name: &str) -> Result<Rc<KyaObject>, Error> {
        if let Some(object) = self.objects.borrow().get(name) {
            return Ok(object.clone());
        }

        Err(Error::RuntimeError(format!(
            "Module '{}' has no attribute '{}'",
            self.name, name
        )))
    }

    fn set_attribute(&self, name: String, value: Rc<KyaObject>) {
        self.objects.borrow_mut().register(name, value);
    }

    fn name(&self) -> String {
        "Module".to_string()
    }

    fn assign(&self, value: Rc<KyaObject>) -> Result<Rc<KyaObject>, Error> {
        Err(Error::TypeError(format!(
            "Cannot assign {} to module attribute",
            value.repr()
        )))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    pub objects: RefCell<HashMap<String, Rc<KyaObject>>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            objects: RefCell::new(HashMap::new()),
        }
    }

    pub fn get(&self, name: &str) -> Option<Rc<KyaObject>> {
        if let Some(object) = self.objects.borrow().get(name) {
            Some(object.clone())
        } else {
            None
        }
    }

    pub fn register(&mut self, name: String, object: Rc<KyaObject>) {
        self.objects.borrow_mut().insert(name, object);
    }

    pub fn keys(&self) -> Vec<String> {
        self.objects.borrow().keys().cloned().collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KyaList {
    pub items: RefCell<Vec<Rc<KyaObject>>>,
    pub instance: Rc<KyaObject>,
}

impl KyaList {
    pub fn get(&self, index: usize) -> Option<Rc<KyaObject>> {
        self.items.borrow().get(index).cloned()
    }

    pub fn len(&self) -> usize {
        self.items.borrow().len()
    }
}

impl ObjectType for KyaList {
    fn get_attribute(&self, name: &str) -> Result<Rc<KyaObject>, Error> {
        self.instance.get_attribute(name)
    }

    fn set_attribute(&self, name: String, value: Rc<KyaObject>) {
        self.instance.set_attribute(name, value);
    }

    fn name(&self) -> String {
        "List".to_string()
    }

    fn assign(&self, value: Rc<KyaObject>) -> Result<Rc<KyaObject>, Error> {
        Err(Error::TypeError(format!(
            "Cannot assign {} to list attribute",
            value.repr()
        )))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KyaSocket {
    pub socket: Option<Socket>,
    pub context: RefCell<Context>,
}

impl KyaSocket {
    pub fn new(socket: Option<Socket>, context: RefCell<Context>) -> Self {
        KyaSocket { socket, context }
    }
}

impl ObjectType for KyaSocket {
    fn get_attribute(&self, name: &str) -> Result<Rc<KyaObject>, Error> {
        if let Some(object) = self.context.borrow().get(name) {
            return Ok(object.clone());
        }

        Err(Error::RuntimeError(format!(
            "Socket has no attribute '{}'",
            name
        )))
    }

    fn set_attribute(&self, name: String, value: Rc<KyaObject>) {
        self.context.borrow_mut().register(name, value);
    }

    fn name(&self) -> String {
        "Socket".to_string()
    }

    fn assign(&self, value: Rc<KyaObject>) -> Result<Rc<KyaObject>, Error> {
        Err(Error::TypeError(format!(
            "Cannot assign {} to socket attribute",
            value.repr()
        )))
    }
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

pub fn unpack_args(
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
        return Ok(arg.clone());
    }

    Err(Error::TypeError(format!(
        "Expected an argument at index {}, but none was provided",
        index
    )))
}

pub fn call_constructor(
    interpreter: &mut Interpreter,
    instance: Rc<KyaObject>,
    args: Vec<Rc<KyaObject>>,
) -> Result<Rc<KyaObject>, Error> {
    let init_args = args.clone();
    let constructor = instance.get_attribute("constructor");

    match constructor {
        Ok(constructor) => {
            let method = Rc::new(KyaObject::Method(KyaMethod {
                function: constructor,
                instance: instance.clone(),
            }));

            return method.call(interpreter, init_args);
        }
        Err(_) => {
            if args.is_empty() {
                return Ok(Rc::new(KyaObject::None(KyaNone {})));
            } else {
                return Err(Error::TypeError(format!(
                    "Constructor for {} does not accept arguments",
                    instance.name()
                )));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context() {
        let context = Context::new();
        assert_eq!(context.objects.borrow().len(), 0);
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
        let mut interpreter = Interpreter::new("".to_string(), ".".to_string());
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
        assert_eq!(kya_frame.locals.objects.borrow().len(), 0);
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
