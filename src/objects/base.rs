use std::cell::RefCell;
use std::rc::Rc;

use crate::ast;
use crate::errors::Error;
use crate::interpreter::{Interpreter, METHOD_TYPE};
use crate::objects::bool_object::BoolObject;
use crate::objects::bytes_object::BytesObject;
use crate::objects::class_object::{
    class_nb_bool, class_tp_call, class_tp_new, class_tp_repr, ClassObject,
};
use crate::objects::function_object::FunctionObject;
use crate::objects::instance_object::InstanceObject;
use crate::objects::method_object::MethodObject;
use crate::objects::modules::sockets::connection_object::ConnectionObject;
use crate::objects::modules::sockets::socket_object::SocketObject;
use crate::objects::none_object::NoneObject;
use crate::objects::number_object::NumberObject;
use crate::objects::rs_function_object::RsFunctionObject;
use crate::objects::string_object::StringObject;

pub type KyaObjectRef = Rc<RefCell<KyaObject>>;
pub type TypeRef = Rc<RefCell<Type>>;
pub type DictRef = Rc<RefCell<std::collections::HashMap<String, KyaObjectRef>>>;
pub type TypeDictRef = Rc<RefCell<std::collections::HashMap<String, TypeRef>>>;
pub type CallableFunctionPtr = fn(
    interpreter: &mut Interpreter,
    callable: KyaObjectRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error>;
pub type TypeFunctionPtr = fn(
    interpreter: &mut Interpreter,
    ob_type: TypeRef,
    args: &mut Vec<KyaObjectRef>,
    receiver: Option<KyaObjectRef>,
) -> Result<KyaObjectRef, Error>;
pub type GetAttrFunctionPtr = fn(
    interpreter: &mut Interpreter,
    obj: KyaObjectRef,
    attr_name: String,
) -> Result<KyaObjectRef, Error>;
pub type NumberCheckFunctionPtr =
    fn(interpreter: &mut Interpreter, obj: KyaObjectRef) -> Result<f64, Error>;
pub type LenFunctionPtr =
    fn(interpreter: &mut Interpreter, obj: KyaObjectRef) -> Result<usize, Error>;
pub type CompareFunctionPtr = fn(
    interpreter: &mut Interpreter,
    obj1: KyaObjectRef,
    obj2: KyaObjectRef,
    operator: ast::Operator,
) -> Result<KyaObjectRef, Error>;
pub type SetAttrFunctionPtr = fn(
    interpreter: &mut Interpreter,
    obj: KyaObjectRef,
    attr_name: String,
    value: KyaObjectRef,
) -> Result<(), Error>;

pub enum KyaObject {
    NoneObject(NoneObject),
    StringObject(StringObject),
    RsFunctionObject(RsFunctionObject),
    FunctionObject(FunctionObject),
    NumberObject(NumberObject),
    ClassObject(ClassObject),
    InstanceObject(InstanceObject),
    MethodObject(MethodObject),
    SocketObject(SocketObject),
    ConnectionObject(ConnectionObject),
    BytesObject(BytesObject),
    BoolObject(BoolObject),
}

pub trait KyaObjectTrait {
    fn get_type(&self) -> TypeRef;
}

pub struct Type {
    pub ob_type: Option<TypeRef>,
    pub name: String,
    pub tp_repr: Option<CallableFunctionPtr>,
    pub tp_call: Option<CallableFunctionPtr>,
    pub tp_set_attr: Option<SetAttrFunctionPtr>,
    pub tp_new: Option<TypeFunctionPtr>,
    pub tp_init: Option<CallableFunctionPtr>,
    pub tp_get_attr: Option<GetAttrFunctionPtr>,
    pub nb_bool: Option<NumberCheckFunctionPtr>,
    pub sq_len: Option<LenFunctionPtr>,
    pub tp_compare: Option<CompareFunctionPtr>,
    pub dict: DictRef,
}

impl Type {
    pub fn as_ref(type_obj: Type) -> TypeRef {
        Rc::new(RefCell::new(type_obj))
    }

    pub fn ready(&mut self) -> Result<(), Error> {
        let parent = self.parent()?;
        let parent_type = parent.borrow();

        if self.tp_repr.is_none() {
            self.tp_repr = parent_type.tp_repr.clone();
        }

        if self.tp_call.is_none() {
            self.tp_call = parent_type.tp_call.clone();
        }

        if self.tp_new.is_none() {
            self.tp_new = parent_type.tp_new.clone();
        }

        if self.tp_init.is_none() {
            self.tp_init = parent_type.tp_init.clone();
        }

        if self.tp_get_attr.is_none() {
            self.tp_get_attr = parent_type.tp_get_attr.clone();
        }

        if self.tp_set_attr.is_none() {
            self.tp_set_attr = parent_type.tp_set_attr.clone();
        }

        if self.nb_bool.is_none() {
            self.nb_bool = parent_type.nb_bool.clone();
        }

        if self.sq_len.is_none() {
            self.sq_len = parent_type.sq_len.clone();
        }

        if self.tp_compare.is_none() {
            self.tp_compare = parent_type.tp_compare.clone();
        }

        Ok(())
    }

    pub fn repr(
        &self,
        interpreter: &mut Interpreter,
        callable: KyaObjectRef,
        args: &mut Vec<KyaObjectRef>,
        receiver: Option<KyaObjectRef>,
    ) -> Result<KyaObjectRef, Error> {
        if let Some(repr_fn) = self.tp_repr {
            let obj = repr_fn(interpreter, callable, args, receiver.clone())?;

            if let KyaObject::StringObject(_) = &*obj.borrow() {
                Ok(obj.clone())
            } else {
                Err(Error::RuntimeError(format!(
                    "__repr__ returned a non-string object (type '{}')",
                    obj.borrow().get_type()?.borrow().name
                )))
            }
        } else {
            Err(Error::RuntimeError("No repr function defined".to_string()))
        }
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        callable: KyaObjectRef,
        args: &mut Vec<KyaObjectRef>,
        receiver: Option<KyaObjectRef>,
    ) -> Result<KyaObjectRef, Error> {
        if let Some(callable_fn) = self.tp_call {
            callable_fn(interpreter, callable, args, receiver)
        } else {
            Err(Error::RuntimeError(format!(
                "The object '{}' is not callable",
                self.name
            )))
        }
    }

    pub fn new(
        &self,
        interpreter: &mut Interpreter,
        ob_type: TypeRef,
        args: &mut Vec<KyaObjectRef>,
        receiver: Option<KyaObjectRef>,
    ) -> Result<KyaObjectRef, Error> {
        if let Some(new_fn) = self.tp_new {
            new_fn(interpreter, ob_type, args, receiver)
        } else {
            Err(Error::RuntimeError(format!(
                "The object '{}' cannot be instantiated",
                self.name
            )))
        }
    }

    pub fn init(
        &self,
        interpreter: &mut Interpreter,
        obj: KyaObjectRef,
        args: &mut Vec<KyaObjectRef>,
        receiver: Option<KyaObjectRef>,
    ) -> Result<KyaObjectRef, Error> {
        if let Some(init_fn) = self.tp_init {
            init_fn(interpreter, obj, args, receiver)
        } else {
            Err(Error::RuntimeError(format!(
                "The object '{}' cannot be initialized",
                self.name
            )))
        }
    }

    pub fn get_attr(
        &self,
        interpreter: &mut Interpreter,
        obj: KyaObjectRef,
        attr_name: String,
    ) -> Result<KyaObjectRef, Error> {
        if let Some(get_attr_fn) = self.tp_get_attr {
            get_attr_fn(interpreter, obj, attr_name)
        } else {
            Err(Error::RuntimeError(format!(
                "The object '{}' has no attribute '{}'",
                self.name, attr_name
            )))
        }
    }

    pub fn set_attr(
        &self,
        interpreter: &mut Interpreter,
        obj: KyaObjectRef,
        attr_name: String,
        value: KyaObjectRef,
    ) -> Result<(), Error> {
        if let Some(set_attr_fn) = self.tp_set_attr {
            set_attr_fn(interpreter, obj, attr_name, value)
        } else {
            Err(Error::RuntimeError(format!(
                "The object '{}' cannot set attribute '{}'",
                self.name, attr_name
            )))
        }
    }

    pub fn nb_bool(&self, interpreter: &mut Interpreter, obj: KyaObjectRef) -> Result<f64, Error> {
        if let Some(nb_bool_fn) = self.nb_bool {
            Ok(nb_bool_fn(interpreter, obj)?)
        } else {
            Err(Error::RuntimeError(format!(
                "The object '{}' does not support boolean conversion",
                self.name
            )))
        }
    }

    pub fn sq_len(&self, interpreter: &mut Interpreter, obj: KyaObjectRef) -> Result<usize, Error> {
        if let Some(sq_len_fn) = self.sq_len {
            sq_len_fn(interpreter, obj)
        } else {
            Err(Error::RuntimeError(format!(
                "The object '{}' does not support length calculation",
                self.name
            )))
        }
    }

    pub fn tp_compare(
        &self,
        interpreter: &mut Interpreter,
        obj1: KyaObjectRef,
        obj2: KyaObjectRef,
        operator: ast::Operator,
    ) -> Result<KyaObjectRef, Error> {
        if let Some(compare_fn) = self.tp_compare {
            compare_fn(interpreter, obj1, obj2, operator)
        } else {
            Err(Error::RuntimeError(format!(
                "The object '{}' does not support comparison",
                self.name
            )))
        }
    }

    pub fn parent(&self) -> Result<TypeRef, Error> {
        if let Some(parent_type) = &self.ob_type {
            Ok(parent_type.clone())
        } else {
            Err(Error::RuntimeError("Type has no parent".to_string()))
        }
    }
}

impl KyaObject {
    pub fn as_object_ref(&self) -> Option<&dyn KyaObjectTrait> {
        match self {
            KyaObject::NoneObject(obj) => Some(obj),
            KyaObject::StringObject(obj) => Some(obj),
            KyaObject::RsFunctionObject(obj) => Some(obj),
            KyaObject::FunctionObject(obj) => Some(obj),
            KyaObject::NumberObject(obj) => Some(obj),
            KyaObject::ClassObject(obj) => Some(obj),
            KyaObject::InstanceObject(obj) => Some(obj),
            KyaObject::MethodObject(obj) => Some(obj),
            KyaObject::SocketObject(obj) => Some(obj),
            KyaObject::ConnectionObject(obj) => Some(obj),
            KyaObject::BytesObject(obj) => Some(obj),
            KyaObject::BoolObject(obj) => Some(obj),
            _ => None,
        }
    }

    pub fn is_instance_of(&self, type_ref: &TypeRef) -> Result<bool, Error> {
        if let Some(obj) = self.as_object_ref() {
            let mut root_type = obj.get_type();
            let mut parent_type = type_ref.borrow().parent()?;

            loop {
                if Rc::ptr_eq(&root_type, type_ref) {
                    return Ok(true);
                }

                if Rc::ptr_eq(&root_type, &parent_type) {
                    return Ok(false);
                }

                root_type = parent_type.clone();

                let new_parent_type = root_type.borrow().parent()?;

                parent_type = new_parent_type;
            }
        } else {
            Ok(false)
        }
    }

    pub fn get_type(&self) -> Result<TypeRef, Error> {
        if let Some(obj) = self.as_object_ref() {
            Ok(obj.get_type())
        } else {
            Err(Error::RuntimeError(
                "Object does not implement KyaObjectTrait".to_string(),
            ))
        }
    }

    pub fn as_ref(object: KyaObject) -> KyaObjectRef {
        Rc::new(RefCell::new(object))
    }

    pub fn from_none_object(none_object: NoneObject) -> KyaObjectRef {
        KyaObject::as_ref(KyaObject::NoneObject(none_object))
    }

    pub fn from_rs_function_object(rs_function_object: RsFunctionObject) -> KyaObjectRef {
        KyaObject::as_ref(KyaObject::RsFunctionObject(rs_function_object))
    }

    pub fn from_function_object(function_object: FunctionObject) -> KyaObjectRef {
        KyaObject::as_ref(KyaObject::FunctionObject(function_object))
    }

    pub fn from_string_object(string_object: StringObject) -> KyaObjectRef {
        KyaObject::as_ref(KyaObject::StringObject(string_object))
    }

    pub fn from_number_object(number_object: NumberObject) -> KyaObjectRef {
        KyaObject::as_ref(KyaObject::NumberObject(number_object))
    }

    pub fn from_class_object(class_object: ClassObject) -> KyaObjectRef {
        KyaObject::as_ref(KyaObject::ClassObject(class_object))
    }

    pub fn from_instance_object(instance_object: InstanceObject) -> KyaObjectRef {
        KyaObject::as_ref(KyaObject::InstanceObject(instance_object))
    }

    pub fn from_method_object(method_object: MethodObject) -> KyaObjectRef {
        KyaObject::as_ref(KyaObject::MethodObject(method_object))
    }

    pub fn from_socket_object(socket_object: SocketObject) -> KyaObjectRef {
        KyaObject::as_ref(KyaObject::SocketObject(socket_object))
    }

    pub fn from_connection_object(connection_object: ConnectionObject) -> KyaObjectRef {
        KyaObject::as_ref(KyaObject::ConnectionObject(connection_object))
    }

    pub fn from_bytes_object(bytes_object: BytesObject) -> KyaObjectRef {
        KyaObject::as_ref(KyaObject::BytesObject(bytes_object))
    }

    pub fn from_bool_object(bool_object: BoolObject) -> KyaObjectRef {
        KyaObject::as_ref(KyaObject::BoolObject(bool_object))
    }
}

impl Default for Type {
    fn default() -> Self {
        Type {
            ob_type: None,
            name: "Unknown".to_string(),
            tp_repr: Some(class_tp_repr),
            tp_call: Some(class_tp_call),
            tp_new: Some(class_tp_new),
            tp_init: None,
            tp_get_attr: Some(generic_get_attr),
            tp_set_attr: Some(generic_set_attr),
            nb_bool: Some(class_nb_bool),
            sq_len: None,
            tp_compare: None,
            dict: Rc::new(RefCell::new(std::collections::HashMap::new())),
        }
    }
}

pub fn create_type_type() -> TypeRef {
    let type_object = Type {
        name: "Type".to_string(),
        ..Default::default()
    };

    let type_ref = Type::as_ref(type_object);

    type_ref.borrow_mut().ob_type = Some(type_ref.clone());

    type_ref
}

pub fn generic_get_attr(
    interpreter: &mut Interpreter,
    obj: KyaObjectRef,
    attr_name: String,
) -> Result<KyaObjectRef, Error> {
    let found_object = get_attr_helper(obj.clone(), attr_name.clone())?;

    if let KyaObject::FunctionObject(_) = &*found_object.borrow() {
        return Ok(KyaObject::from_method_object(MethodObject {
            ob_type: interpreter.get_type(METHOD_TYPE),
            instance_object: obj.clone(),
            function: found_object.clone(),
        }));
    } else if let KyaObject::RsFunctionObject(_) = &*found_object.borrow() {
        return Ok(KyaObject::from_method_object(MethodObject {
            ob_type: interpreter.get_type(METHOD_TYPE),
            instance_object: obj.clone(),
            function: found_object.clone(),
        }));
    }

    Ok(found_object)
}

fn get_attr_helper(object: KyaObjectRef, attr_name: String) -> Result<KyaObjectRef, Error> {
    let ob_type = object.borrow().get_type()?;

    if let Some(attr) = ob_type.borrow().dict.borrow().get(&attr_name) {
        return Ok(attr.clone());
    } else {
        let mut root_type = ob_type;
        let mut parent_type = root_type.borrow().parent()?;

        loop {
            if Rc::ptr_eq(&root_type, &parent_type) {
                break;
            }

            if let Some(attr) = root_type.borrow().dict.borrow().get(&attr_name) {
                return Ok(attr.clone());
            }

            root_type = parent_type.clone();

            let new_parent_type = root_type.borrow().parent()?;

            parent_type = new_parent_type;
        }
    }

    Err(Error::RuntimeError(format!(
        "The object '{}' has no attribute '{}'",
        object.borrow().get_type()?.borrow().name,
        attr_name
    )))
}

pub fn generic_set_attr(
    _interpreter: &mut Interpreter,
    obj: KyaObjectRef,
    attr_name: String,
    value: KyaObjectRef,
) -> Result<(), Error> {
    let ob_type = obj.borrow().get_type()?;

    ob_type.borrow().dict.borrow_mut().insert(attr_name, value);

    Ok(())
}
