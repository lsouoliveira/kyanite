use crate::objects::base::KyaObjectRef;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    LoadConst = 0,
    StoreName = 1,
    LoadName = 2,
    Call = 3,
    PopTop = 4,
    MakeFunction = 5,
    LoadAttr = 6,
    Compare = 7,
    JumpBack = 8,
    PopAndJumpIfFalse = 9,
    Jump = 10,
    MakeClass = 11,
    StoreAttr = 12,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOperator {
    Equal = 0,
}

impl ComparisonOperator {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ComparisonOperator::Equal),
            _ => None,
        }
    }
}

impl std::fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparisonOperator::Equal => write!(f, "EQUAL"),
        }
    }
}

impl Opcode {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Opcode::LoadConst),
            1 => Some(Opcode::StoreName),
            2 => Some(Opcode::LoadName),
            3 => Some(Opcode::Call),
            4 => Some(Opcode::PopTop),
            5 => Some(Opcode::MakeFunction),
            6 => Some(Opcode::LoadAttr),
            7 => Some(Opcode::Compare),
            8 => Some(Opcode::JumpBack),
            9 => Some(Opcode::PopAndJumpIfFalse),
            10 => Some(Opcode::Jump),
            11 => Some(Opcode::MakeClass),
            12 => Some(Opcode::StoreAttr),
            _ => None,
        }
    }
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcode::LoadConst => write!(f, "LOAD_CONST"),
            Opcode::StoreName => write!(f, "STORE_NAME"),
            Opcode::LoadName => write!(f, "LOAD_NAME"),
            Opcode::Call => write!(f, "CALL_FUNCTION"),
            Opcode::PopTop => write!(f, "POP_TOP"),
            Opcode::MakeFunction => write!(f, "MAKE_FUNCTION"),
            Opcode::LoadAttr => write!(f, "LOAD_ATTR"),
            Opcode::Compare => write!(f, "COMPARE"),
            Opcode::JumpBack => write!(f, "JUMP_BACK"),
            Opcode::PopAndJumpIfFalse => write!(f, "POP_AND_JUMP_IF_FALSE"),
            Opcode::Jump => write!(f, "JUMP"),
            Opcode::MakeClass => write!(f, "MAKE_CLASS"),
            Opcode::StoreAttr => write!(f, "STORE_ATTR"),
        }
    }
}

pub struct CodeObject {
    pub code: Vec<u8>,
    pub consts: Vec<KyaObjectRef>,
    pub names: Vec<String>,
    pub args: Vec<String>,
    pub name: String,
}

impl Clone for CodeObject {
    fn clone(&self) -> Self {
        CodeObject {
            code: self.code.clone(),
            consts: self.consts.clone(),
            names: self.names.clone(),
            args: self.args.clone(),
            name: self.name.clone(),
        }
    }
}

impl CodeObject {
    pub fn new() -> Self {
        CodeObject {
            code: Vec::new(),
            consts: Vec::new(),
            names: Vec::new(),
            args: Vec::new(),
            name: String::new(),
        }
    }

    pub fn add_instruction(&mut self, opcode: u8) {
        self.code.push(opcode);
    }

    pub fn add_const(&mut self, const_value: KyaObjectRef) -> u8 {
        self.consts.push(const_value);
        (self.consts.len() - 1) as u8
    }

    pub fn add_name(&mut self, name: String) -> u8 {
        for (index, existing_name) in self.names.iter().enumerate() {
            if existing_name == &name {
                return index as u8;
            }
        }

        self.names.push(name);
        (self.names.len() - 1) as u8
    }

    pub fn instructions_count(&self) -> usize {
        self.code.len()
    }

    pub fn instruction_at(&self, offset: usize) -> u8 {
        if offset < self.code.len() {
            self.code[offset]
        } else {
            panic!("Offset out of bounds")
        }
    }

    pub fn set_instruction_at(&mut self, offset: usize, value: u8) {
        if offset < self.code.len() {
            self.code[offset] = value;
        } else {
            panic!("Offset out of bounds");
        }
    }

    pub fn dis(&self) -> String {
        let mut disassembler = Disassembler::new(self.clone());
        disassembler.disassemble();
        disassembler.output
    }
}

struct Disassembler {
    output: String,
    code_object: CodeObject,
}

impl Disassembler {
    pub fn new(code_object: CodeObject) -> Self {
        Disassembler {
            output: String::new(),
            code_object,
        }
    }

    pub fn disassemble(&mut self) {
        let mut pc: u8 = 0;

        while pc < self.instructions_count() as u8 {
            let opcode = self.instruction_at(pc.into());

            self.output.push_str(&format!("{:04}: ", pc));

            match opcode {
                0 => {
                    pc = self.write_load_const(pc);
                }
                1 => {
                    pc = self.write_store_name(pc);
                }
                2 => {
                    pc = self.write_load_name(pc);
                }
                3 => {
                    pc = self.write_call_function(pc);
                }
                4 => {
                    pc = self.write_pop_top(pc);
                }
                5 => {
                    pc = self.write_make_function(pc);
                }
                6 => {
                    pc = self.write_load_attr(pc);
                }
                7 => {
                    pc = self.write_compare(pc);
                }
                8 => {
                    pc = self.write_jump_back(pc);
                }
                9 => {
                    pc = self.write_jump_if_false(pc);
                }
                10 => {
                    pc = self.write_jump(pc);
                }
                11 => {
                    pc = self.write_make_class(pc);
                }
                _ => {
                    panic!("Unknown opcode: {}", opcode);
                }
            }

            if pc < self.instructions_count() as u8 {
                self.output.push('\n');
            }
        }
    }

    fn instructions_count(&self) -> usize {
        self.code_object.code.len()
    }

    fn instruction_at(&self, offset: usize) -> u8 {
        if offset < self.code_object.code.len() {
            self.code_object.code[offset]
        } else {
            panic!("Offset out of bounds")
        }
    }

    fn write_load_const(&mut self, pc: u8) -> u8 {
        let const_index = self.instruction_at((pc + 1).into());

        self.output.push_str(&format!("LOAD_CONST {}", const_index));

        pc + 2
    }

    fn write_store_name(&mut self, pc: u8) -> u8 {
        let name_index = self.instruction_at((pc + 1).into());

        self.output.push_str(&format!("STORE_NAME {}", name_index));

        pc + 2
    }

    fn write_load_name(&mut self, pc: u8) -> u8 {
        let name_index = self.instruction_at((pc + 1).into());
        let name = self
            .code_object
            .names
            .get(name_index as usize)
            .expect("Name index out of bounds");

        self.output
            .push_str(&format!("LOAD_NAME {} ({})", name_index, name));

        pc + 2
    }

    fn write_call_function(&mut self, pc: u8) -> u8 {
        let arg_count = self.instruction_at((pc + 1).into());

        self.output
            .push_str(&format!("CALL_FUNCTION {}", arg_count));

        pc + 2
    }

    fn write_pop_top(&mut self, pc: u8) -> u8 {
        self.output.push_str("POP_TOP");
        pc + 1
    }

    fn write_make_function(&mut self, pc: u8) -> u8 {
        self.output.push_str("MAKE_FUNCTION");
        pc + 1
    }

    fn write_load_attr(&mut self, pc: u8) -> u8 {
        let attr_index = self.instruction_at((pc + 1).into());
        let attr_name = self
            .code_object
            .names
            .get(attr_index as usize)
            .expect("Attribute index out of bounds");

        self.output
            .push_str(&format!("LOAD_ATTR {} ({})", attr_index, attr_name));

        pc + 2
    }

    fn write_compare(&mut self, pc: u8) -> u8 {
        let op_index = self.instruction_at((pc + 1).into());
        let op = ComparisonOperator::from_u8(op_index).expect("Invalid comparison operation index");

        self.output.push_str(&format!("COMPARE {}", op));

        pc + 2
    }

    fn write_jump_back(&mut self, pc: u8) -> u8 {
        let offset = self.instruction_at((pc + 1).into());
        self.output.push_str(&format!("JUMP_BACK {}", offset));
        pc + 2
    }

    fn write_jump_if_false(&mut self, pc: u8) -> u8 {
        let offset = self.instruction_at((pc + 1).into());
        self.output.push_str(&format!("JUMP_IF_FALSE {}", offset));
        pc + 2
    }

    fn write_jump(&mut self, pc: u8) -> u8 {
        let offset = self.instruction_at((pc + 1).into());
        self.output.push_str(&format!("JUMP {}", offset));
        pc + 2
    }

    pub fn write_make_class(&mut self, pc: u8) -> u8 {
        self.output.push_str("MAKE_CLASS");
        pc + 1
    }
}
