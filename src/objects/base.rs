use std::cell::RefCell;
use std::rc::Rc;

use crate::errors::Error;
use crate::interpreter::Interpreter;
use crate::objects::class_object::ClassObject;
use crate::objects::function_object::FunctionObject;
use crate::objects::instance_object::InstanceObject;
use crate::objects::none_object::NoneObject;
use crate::objects::number_object::NumberObject;
use crate::objects::rs_function_object::RsFunctionObject;
use crate::objects::string_object::StringObject;

pub type KyaObjectRef = Rc<RefCell<KyaObject>>;
pub type TypeRef = Rc<RefCell<Type>>;
pub type DictRef = Rc<RefCell<std::collections::HashMap<String, KyaObjectRef>>>;
pub type CallableFunctionPtr = fn(
    interpreter: &mut Interpreter,
    callable: KyaObjectRef,
    args: Vec<KyaObjectRef>,
) -> Result<KyaObjectRef, Error>;
pub type TypeFunctionPtr = fn(
    interpreter: &mut Interpreter,
    ob_type: TypeRef,
    args: Vec<KyaObjectRef>,
) -> Result<KyaObjectRef, Error>;
pub type GetAttrFunctionPtr = fn(
    interpreter: &mut Interpreter,
    obj: KyaObjectRef,
    attr_name: String,
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
    pub dict: DictRef,
}

impl Type {
    pub fn as_ref(type_obj: Type) -> TypeRef {
        Rc::new(RefCell::new(type_obj))
    }

    pub fn repr(
        &self,
        interpreter: &mut Interpreter,
        callable: KyaObjectRef,
        args: Vec<KyaObjectRef>,
    ) -> Result<KyaObjectRef, Error> {
        if let Some(repr_fn) = self.tp_repr {
            let obj = repr_fn(interpreter, callable, args)?;

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
        args: Vec<KyaObjectRef>,
    ) -> Result<KyaObjectRef, Error> {
        if let Some(callable_fn) = self.tp_call {
            callable_fn(interpreter, callable, args)
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
        args: Vec<KyaObjectRef>,
    ) -> Result<KyaObjectRef, Error> {
        if let Some(new_fn) = self.tp_new {
            new_fn(interpreter, ob_type, args)
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
        args: Vec<KyaObjectRef>,
    ) -> Result<KyaObjectRef, Error> {
        if let Some(init_fn) = self.tp_init {
            init_fn(interpreter, obj, args)
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
            _ => None,
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
}

impl Default for Type {
    fn default() -> Self {
        Type {
            ob_type: None,
            name: "Unknown".to_string(),
            tp_repr: None,
            tp_call: None,
            tp_new: None,
            tp_init: None,
            tp_get_attr: None,
            tp_set_attr: None,
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
