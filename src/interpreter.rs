use crate::builtins::methods::kya_print;
use crate::bytecode::{CodeObject, Opcode};
use crate::errors::Error;
use crate::objects::bool_object::BOOL_TYPE;
// use crate::objects::bytes_object::create_bytes_type;
use crate::objects::class_object::{class_new, ClassObject};
// use crate::objects::function_object::{create_function_type, FunctionObject};
// use crate::objects::method_object::create_method_type;
// use crate::objects::modules::sockets::connection_object::create_connection_type;
// use crate::objects::modules::sockets::functions::kya_socket;
// use crate::objects::modules::sockets::socket_object::create_socket_type;
use crate::objects::none_object::{none_new, NONE_TYPE};
use crate::objects::number_object::{kya_compare_numbers, NumberObject};
use crate::objects::rs_function_object::rs_function_new;
use crate::objects::string_object::StringObject;
use crate::opcodes::OPCODE_HANDLERS;
// use crate::objects::utils::{create_rs_function_object, kya_is_true};
use crate::parser;
use crate::visitor::Evaluator;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::objects::base::{
    DictRef, KyaObject, KyaObjectRef, Type, TypeDictRef, TypeRef, BASE_TYPE,
};

type FrameRef = Arc<Mutex<Frame>>;

pub struct Interpreter {
    root: PathBuf,
    frames: Vec<FrameRef>,
}

pub struct Frame {
    pub locals: DictRef,
    pub globals: DictRef,
    pub code: CodeObject,
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

    pub fn set_current_pc(&mut self, pc: usize) {
        self.pc = pc;
    }

    pub fn increment_pc(&mut self, offset: usize) {
        self.pc = self.pc + offset;
    }

    pub fn next_opcode(&self) -> u8 {
        if self.pc < self.code.code.len() {
            return self.code.instruction_at(self.pc);
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
}

fn register_builtin_types(frame: &mut Frame) {
    let type_object = class_new(BASE_TYPE.clone());
    let none_type = class_new(NONE_TYPE.clone());
    // // let rs_function_type = class_new(RS_FUNCTION_OBJE);

    frame.register_local("Type", type_object);
    frame.register_local("None", none_type);
    // frame.register_local(RS_FUNCTION_TYPE, rs_function_type);
}

fn register_builtins(frame: &mut Frame) {
    register_builtin_types(frame);
    register_builtin_objects(frame);
}

fn create_main_frame(code: CodeObject) -> FrameRef {
    let globals = Arc::new(Mutex::new(HashMap::new()));
    let frame_ref = Arc::new(Mutex::new(Frame {
        locals: globals.clone(),
        globals,
        code,
        pc: 0,
        stack: vec![],
    }));

    register_builtins(&mut frame_ref.lock().unwrap());

    frame_ref
}

impl Interpreter {
    pub fn new(root: &str) -> Self {
        let root_path = PathBuf::from(root);

        Interpreter {
            root: root_path,
            frames: vec![],
        }
    }

    pub fn current_frame(&self) -> FrameRef {
        self.frames.last().unwrap().clone()
    }

    pub fn print_frames(&self) {
        for (i, frame) in self.frames.iter().enumerate() {
            for (name, _) in frame.lock().unwrap().locals.lock().unwrap().iter() {
                println!("Frame {}: {}", i, name,);
            }
        }
    }

    pub fn push_frame(&mut self, frame: FrameRef) {
        self.frames.push(frame);
    }

    pub fn push_empty_frame(&mut self) {
        let frame_ref = Arc::new(Mutex::new(Frame {
            locals: Arc::new(Mutex::new(HashMap::new())),
            globals: self.current_frame().lock().unwrap().globals.clone(),
            code: CodeObject {
                consts: vec![],
                names: vec![],
                code: vec![],
            },
            pc: 0,
            stack: vec![],
        }));

        self.push_frame(frame_ref);
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop();
    }

    pub fn eval(&mut self, code_object: &CodeObject) -> Result<KyaObjectRef, Error> {
        let frame = create_main_frame(code_object.clone());

        let result = self.eval_frame(&mut frame.lock().unwrap())?;

        Ok(result)
    }

    fn eval_frame(&mut self, frame: &mut Frame) -> Result<KyaObjectRef, Error> {
        while frame.current_pc() < frame.current_code_length() {
            let opcode = frame.next_opcode();

            frame.increment_pc(1);

            OPCODE_HANDLERS[opcode as usize](frame)?;
        }

        Ok(frame.resolve("None")?)
    }
}
