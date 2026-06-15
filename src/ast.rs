use crate::error::BasmError;
use crate::numerics::{u5, i26};

#[derive(Debug)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    kind: InstructionKind,
    parameter: Vec<Parameter>,
}

impl Instruction {
    pub fn new(kind: InstructionKind, parameter: Vec<Parameter>) -> Self {
        Self {
            kind,
            parameter,
        }
    }
}

#[derive(Debug)]
pub enum InstructionKind {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,

    // Binary
    And,
    Or,
    Xor,
    Sll,
    Srl,
    Sra,

    // Compare
    Eq,
    Lt,

    // Memory
    Load,
    Store,

    // Stack
    Push,
    Pop,

    // Immediate
    Immh,
    Imml,

    // Control flow
    Jmp,
    JmpIf,
    Call,
    Ret,

    // System
    Halt,

    Nop,
}
impl InstructionKind {
    pub fn is_type1(&self) -> bool {
        match self {
            InstructionKind::Add    |
            InstructionKind::Sub    |
            InstructionKind::Mul    |
            InstructionKind::Div    |
            InstructionKind::And    |
            InstructionKind::Or     |
            InstructionKind::Xor    |
            InstructionKind::Sll    |
            InstructionKind::Srl    |
            InstructionKind::Sra    |
            InstructionKind::Eq     |
            InstructionKind::Lt     |
            InstructionKind::Load   |
            InstructionKind::Store  |
            InstructionKind::Push => true,
            _ => false,
        }
    }

    pub fn is_type0(&self) -> bool {
        !self.is_type1()
    }

    pub fn need_immediate(&self) -> bool {
        match self {
            InstructionKind::Immh   |
            InstructionKind::Imml => true,
            _ => false,
        }
    }

    pub fn need_label(&self) -> bool {
        match self {
            InstructionKind::Jmp    |
            InstructionKind::JmpIf  |
            InstructionKind::Call => true,
            _ => false,
        }
    }

    pub fn nb_parameter(&self) -> usize {
        match self {
            InstructionKind::Add    |
            InstructionKind::Sub    |
            InstructionKind::Mul    |
            InstructionKind::Div    |
            InstructionKind::And    |
            InstructionKind::Or     |
            InstructionKind::Xor    |
            InstructionKind::Sll    |
            InstructionKind::Srl    |
            InstructionKind::Sra    |
            InstructionKind::Eq     |
            InstructionKind::Lt     |
            InstructionKind::Load   |
            InstructionKind::Store => 2,

            InstructionKind::Push   |
            InstructionKind::Immh   |
            InstructionKind::Imml   |
            InstructionKind::Jmp    |
            InstructionKind::JmpIf  |
            InstructionKind::Call => 1,

            _ => 0,
        }
    }

    pub fn get_instruction_kind(name: &str) -> Result<Self, BasmError> {
        match name {
            "add"   => Ok(InstructionKind::Add),
            "sub"   => Ok(InstructionKind::Sub),
            "mul"   => Ok(InstructionKind::Mul),
            "div"   => Ok(InstructionKind::Div),
            "and"   => Ok(InstructionKind::And),
            "or"    => Ok(InstructionKind::Or),
            "xor"   => Ok(InstructionKind::Xor),
            "sll"   => Ok(InstructionKind::Sll),
            "srl"   => Ok(InstructionKind::Srl),
            "sra"   => Ok(InstructionKind::Sra),
            "eq"    => Ok(InstructionKind::Eq),
            "lt"    => Ok(InstructionKind::Lt),
            "load"  => Ok(InstructionKind::Load),
            "store" => Ok(InstructionKind::Store),
            "push"  => Ok(InstructionKind::Push),
            "pop"   => Ok(InstructionKind::Pop),
            "immh"  => Ok(InstructionKind::Immh),
            "imml"  => Ok(InstructionKind::Imml),
            "jmp"   => Ok(InstructionKind::Jmp),
            "jmpif" => Ok(InstructionKind::JmpIf),
            "call"  => Ok(InstructionKind::Call),
            "ret"   => Ok(InstructionKind::Ret),
            "halt"  => Ok(InstructionKind::Halt),
            "nop"   => Ok(InstructionKind::Nop),
            _       => Err(BasmError::CompilationFailed)
        }
    }
}

#[derive(Debug)]
pub enum Parameter {
    BeltIndex(u5),
    Immediate(i26),
}
