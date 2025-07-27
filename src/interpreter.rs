use crate::builtins::methods::kya_print;
use crate::bytecode::CodeObject;
use crate::errors::Error;
use crate::lock::{kya_acquire_lock, kya_release_lock};
use crate::objects::bool_object::bool_new;
use crate::objects::class_object::class_new;
use crate::objects::list_object::LIST_TYPE;
use crate::objects::modules::sockets::functions::kya_socket;
use crate::objects::modules::threads::lock_object::LOCK_TYPE;
use crate::objects::modules::threads::thread_object::THREAD_OBJECT;
use crate::objects::none_object::none_new;
use crate::objects::rs_function_object::rs_function_new;
use crate::objects::string_object::STRING_TYPE;
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

use crate::objects::base::{DictRef, KyaObjectRef, BASE_TYPE};

pub struct Interpreter {
    root: PathBuf,
}

pub struct Frame {
    pub locals: DictRef,
    pub globals: DictRef,
    pub code: Arc<CodeObject>,
    pub pc: usize,
    pub stack: Vec<KyaObjectRef>,
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

    frame.register_local("Type", type_object);
    frame.register_local("String", string_class);
    frame.register_local("Thread", thread_class);
    frame.register_local("List", list_class);
    frame.register_local("Lock", lock_class);

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

        Ok(result)
    }
}

pub fn eval_frame(frame: &mut Frame) -> Result<KyaObjectRef, Error> {
    let mut instructions_processed = 0;

    while frame.current_pc() < frame.current_code_length() {
        if instructions_processed >= 1 {
            instructions_processed = 0;

            kya_release_lock();
            thread::yield_now();
            kya_acquire_lock();
        }

        let opcode = frame.next_opcode();

        OPCODE_HANDLERS[opcode as usize](frame)?;

        instructions_processed += 1;
    }

    if let Some(object) = frame.stack.last() {
        return Ok(object.clone());
    }

    Ok(frame.resolve("None")?)
}
