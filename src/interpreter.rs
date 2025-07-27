use crate::builtins::methods::kya_print;
use crate::bytecode::CodeObject;
use crate::errors::Error;
use crate::lock::{kya_acquire_lock, kya_release_lock};
use crate::objects::bool_object::bool_new;
use crate::objects::class_object::class_new;
use crate::objects::exception_object::{exception_new, EXCEPTION_TYPE};
use crate::objects::hash_object::HASH_TYPE;
use crate::objects::list_object::LIST_TYPE;
use crate::objects::modules::sockets::functions::kya_socket;
use crate::objects::modules::threads::lock_object::LOCK_TYPE;
use crate::objects::modules::threads::thread_object::THREAD_OBJECT;
use crate::objects::none_object::none_new;
use crate::objects::rs_function_object::rs_function_new;
use crate::objects::string_object::{string_new, STRING_TYPE};
use crate::objects::utils::{object_to_string_repr, string_object_to_string};
use crate::opcodes::OPCODE_HANDLERS;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::LazyLock as Lazy;
use std::sync::{Arc, Mutex};
use std::thread;

pub static NONE_OBJECT: Lazy<KyaObjectRef> =
    Lazy::new(|| none_new().expect("Failed to create None object"));
pub static TRUE_OBJECT: Lazy<KyaObjectRef> = Lazy::new(|| bool_new(true));
pub static FALSE_OBJECT: Lazy<KyaObjectRef> = Lazy::new(|| bool_new(false));

use crate::objects::base::{DictRef, KyaObject, KyaObjectRef, BASE_TYPE};

pub struct Interpreter {
    root: PathBuf,
}

pub struct Frame {
    pub locals: DictRef,
    pub globals: DictRef,
    pub code: Arc<CodeObject>,
    pub pc: usize,
    pub stack: Vec<KyaObjectRef>,
    pub return_value: Option<KyaObjectRef>,
    pub error: Option<KyaObjectRef>,
}

impl Frame {
    pub fn register_local(&mut self, name: &str, object: KyaObjectRef) {
        self.locals.lock().unwrap().insert(name.to_string(), object);
    }

    pub fn resolve(&self, name: &str) -> Result<KyaObjectRef, Error> {
        if let Some(object) = self.locals.lock().unwrap().get(name) {
            return Ok(object.clone());
        }

        if let Some(object) = self.globals.lock().unwrap().get(name) {
            return Ok(object.clone());
        }

        Err(Error::RuntimeError(format!(
            "name '{}' is not defined",
            name
        )))
    }

    pub fn get_const(&self, index: usize) -> Option<KyaObjectRef> {
        if index < self.code.consts.len() {
            return Some(self.code.consts[index].clone());
        }

        None
    }

    pub fn get_name(&self, index: usize) -> Option<String> {
        if index < self.code.names.len() {
            return Some(self.code.names[index].clone());
        }

        None
    }

    pub fn current_pc(&self) -> usize {
        self.pc
    }

    pub fn set_pc(&mut self, pc: usize) {
        self.pc = pc;
    }

    pub fn increment_pc(&mut self, offset: usize) {
        self.pc = self.pc + offset;
    }

    pub fn next_opcode(&mut self) -> u8 {
        if self.pc < self.code.code.len() {
            let value = self.code.instruction_at(self.pc);

            self.pc += 1;

            return value;
        }

        panic!(
            "Attempt to read opcode at invalid program counter: {}",
            self.pc
        );
    }

    pub fn current_code_length(&self) -> usize {
        self.code.instructions_count()
    }

    pub fn push_stack(&mut self, object: KyaObjectRef) {
        self.stack.push(object);
    }

    pub fn pop_stack(&mut self) -> Result<KyaObjectRef, Error> {
        if let Some(object) = self.stack.pop() {
            return Ok(object);
        }

        Err(Error::RuntimeError(
            "Attempted to pop from an empty stack".to_string(),
        ))
    }

    pub fn set_return_value(&mut self, value: Option<KyaObjectRef>) {
        self.return_value = value;
    }

    pub fn set_error(&mut self, error: Option<KyaObjectRef>) {
        self.error = error;
    }
}

fn register_builtin_objects(frame: &mut Frame) {
    let print_rs_function_object = rs_function_new(kya_print);

    frame.register_local("print", print_rs_function_object);
    frame.register_local("None", NONE_OBJECT.clone());
    frame.register_local("true", TRUE_OBJECT.clone());
    frame.register_local("false", FALSE_OBJECT.clone());
    frame.register_local("socket", rs_function_new(kya_socket));
}

fn register_builtin_types(frame: &mut Frame) {
    let type_object = class_new(BASE_TYPE.clone());
    let string_class = class_new(STRING_TYPE.clone());
    let thread_class = class_new(THREAD_OBJECT.clone());
    let list_class = class_new(LIST_TYPE.clone());
    let lock_class = class_new(LOCK_TYPE.clone());
    let hash_class = class_new(HASH_TYPE.clone());
    let exception_class = class_new(EXCEPTION_TYPE.clone());

    frame.register_local("Type", type_object);
    frame.register_local("String", string_class);
    frame.register_local("Thread", thread_class);
    frame.register_local("List", list_class);
    frame.register_local("Lock", lock_class);
    frame.register_local("Hash", hash_class);
    frame.register_local("Exception", exception_class);

    // frame.register_local(RS_FUNCTION_TYPE, rs_function_type);
}

fn register_builtins(frame: &mut Frame) {
    register_builtin_types(frame);
    register_builtin_objects(frame);
}

fn create_main_frame(code: CodeObject) -> Frame {
    let globals = Arc::new(Mutex::new(HashMap::new()));
    let mut frame = Frame {
        locals: globals.clone(),
        globals,
        code: Arc::new(code),
        pc: 0,
        stack: vec![],
        return_value: None,
        error: None,
    };

    register_builtins(&mut frame);

    frame
}

impl Interpreter {
    pub fn new(root: &str) -> Self {
        let root_path = PathBuf::from(root);

        Interpreter { root: root_path }
    }

    pub fn eval(&mut self, code_object: &CodeObject) -> Result<KyaObjectRef, Error> {
        kya_acquire_lock();

        let mut frame = create_main_frame(code_object.clone());

        let result = eval_frame(&mut frame)?;

        kya_release_lock();

        Ok(result)
    }
}

pub fn eval_frame(frame: &mut Frame) -> Result<KyaObjectRef, Error> {
    let mut instructions_processed = 0;

    while frame.current_pc() < frame.current_code_length() {
        if instructions_processed >= 100 {
            instructions_processed = 0;

            kya_release_lock();
            thread::yield_now();
            kya_acquire_lock();
        }

        let opcode = frame.next_opcode();

        let result = OPCODE_HANDLERS[opcode as usize](frame);

        if let Err(error) = result {
            if let Error::Exception(_, _) = error {
                return Err(error);
            } else {
                let error_object = map_error_to_exception(error)?;
                handle_exception(error_object.clone())?;
            }
        }

        instructions_processed += 1;

        if let Some(return_value) = &frame.return_value {
            return Ok(return_value.clone());
        }

        if let Some(error) = &frame.error {
            handle_exception(error.clone())?;
        }
    }

    if let Some(object) = frame.stack.last() {
        return Ok(object.clone());
    }

    Ok(frame.resolve("None")?)
}

fn map_error_to_exception(error: Error) -> Result<KyaObjectRef, Error> {
    let message = match error {
        Error::RuntimeError(msg) => msg,
        _ => "An error occurred".to_string(),
    };

    let exception_object = exception_new(string_new(&message));

    Ok(exception_object)
}

fn handle_exception(error: KyaObjectRef) -> Result<KyaObjectRef, Error> {
    kya_release_lock();

    let message = match &*error.lock().unwrap() {
        KyaObject::ExceptionObject(exception) => exception.message.clone(),
        _ => {
            return Err(Error::RuntimeError(
                "Uncaught exception is not an ExceptionObject".to_string(),
            ))
        }
    };

    let ob_type_name = error
        .lock()
        .unwrap()
        .get_type()?
        .lock()
        .unwrap()
        .name
        .clone();

    Err(Error::Exception(
        ob_type_name,
        object_to_string_repr(&message)?,
    ))
}
