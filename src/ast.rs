use crate::error::BasmError;
use crate::numerics::{BeltIdx, Immediate};

#[derive(Debug)]
pub struct Program {
    org: u32,
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn new(org: u32, instructions: Vec<Instruction>) -> Self {
        Self {
            org,
            instructions,
        }
    }
}

impl Into<Vec<u8>> for Program {
    fn into(self) -> Vec<u8> {
        let magic: Vec<u8> = 0xD12EA2E2u32.to_le_bytes().to_vec();
        let org: Vec<u8> = self.org.to_le_bytes().to_vec();

        let instructions: Vec<u8> = self.instructions
            .into_iter()
            .flat_map(|instr| {
                let word: u32 = instr.into();
                word.to_le_bytes()
            })
            .collect();

        [magic, org, instructions].concat()
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

impl Into<u32> for Instruction {
    fn into(self) -> u32 {
        let opcode = self.kind.as_opcode() as u32;

        if self.kind.is_type1() {
            let operand_a = match &self.parameter[0] {
                Parameter::BeltIndex(BeltIdx(v)) => (*v as u32) & 0x1F,
                _ => unreachable!(),
            };

            let operand_b = match self.kind.nb_parameter() {
                2 => match &self.parameter[1] {
                    Parameter::BeltIndex(BeltIdx(v)) => (*v as u32) & 0x1F,
                    _ => unreachable!(),
                },
                _ => 0u32,
            };

            (operand_b << 11) | (operand_a << 6) | opcode
        } else {
            let immediate = match self.kind.nb_parameter() {
                1 => match &self.parameter[0] {
                    Parameter::Immediate(Immediate(v)) => (*v as u32) & 0x03FFFFFF,
                    _ => unreachable!(),
                },
                _ => 0u32,
            };

            (immediate << 6) | opcode
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
    Load8,
    Store8,
    Load16,
    Store16,
    Load32,
    Store32,

    // Scratchpad
    Put,
    Pick,

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
            InstructionKind::Load8  |
            InstructionKind::Store8 |
            InstructionKind::Load16 |
            InstructionKind::Store16|
            InstructionKind::Load32 |
            InstructionKind::Store32|
            InstructionKind::Put    |
            InstructionKind::Pick => true,
            _ => false,
        }
    }

    pub fn is_type0(&self) -> bool {
        !self.is_type1()
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
            InstructionKind::Load8  |
            InstructionKind::Store8 |
            InstructionKind::Load16 |
            InstructionKind::Store16|
            InstructionKind::Load32 |
            InstructionKind::Store32|
            InstructionKind::Put => 2,

            InstructionKind::Pick   |
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
            "add"       => Ok(InstructionKind::Add),
            "sub"       => Ok(InstructionKind::Sub),
            "mul"       => Ok(InstructionKind::Mul),
            "div"       => Ok(InstructionKind::Div),
            "and"       => Ok(InstructionKind::And),
            "or"        => Ok(InstructionKind::Or),
            "xor"       => Ok(InstructionKind::Xor),
            "sll"       => Ok(InstructionKind::Sll),
            "srl"       => Ok(InstructionKind::Srl),
            "sra"       => Ok(InstructionKind::Sra),
            "eq"        => Ok(InstructionKind::Eq),
            "lt"        => Ok(InstructionKind::Lt),
            "put"       => Ok(InstructionKind::Put),
            "pick"      => Ok(InstructionKind::Pick),
            "load8"     => Ok(InstructionKind::Load8),
            "store8"    => Ok(InstructionKind::Store8),
            "load16"    => Ok(InstructionKind::Load16),
            "store16"   => Ok(InstructionKind::Store16),
            "load32"    => Ok(InstructionKind::Load32),
            "store32"   => Ok(InstructionKind::Store32),
            "immh"      => Ok(InstructionKind::Immh),
            "imml"      => Ok(InstructionKind::Imml),
            "jmp"       => Ok(InstructionKind::Jmp),
            "jmpif"     => Ok(InstructionKind::JmpIf),
            "call"      => Ok(InstructionKind::Call),
            "ret"       => Ok(InstructionKind::Ret),
            "halt"      => Ok(InstructionKind::Halt),
            "nop"       => Ok(InstructionKind::Nop),
            _           => Err(BasmError::InvalidInstruction)
        }
    }

    pub fn as_opcode(&self) -> u8 {
        match self {
            InstructionKind::Add        => 0b000001,
            InstructionKind::Sub        => 0b000011,
            InstructionKind::Mul        => 0b000101,
            InstructionKind::Div        => 0b000111,
            InstructionKind::And        => 0b001001,
            InstructionKind::Or         => 0b001011,
            InstructionKind::Xor        => 0b001101,
            InstructionKind::Sll        => 0b001111,
            InstructionKind::Srl        => 0b010001,
            InstructionKind::Sra        => 0b010011,
            InstructionKind::Eq         => 0b010101,
            InstructionKind::Lt         => 0b010111,
            InstructionKind::Put        => 0b011001,
            InstructionKind::Pick       => 0b011011,
            InstructionKind::Load8      => 0b100001,
            InstructionKind::Store8     => 0b100011,
            InstructionKind::Load16     => 0b100101,
            InstructionKind::Store16    => 0b100111,
            InstructionKind::Load32     => 0b101001,
            InstructionKind::Store32    => 0b101011,
            InstructionKind::Immh       => 0b000010,
            InstructionKind::Imml       => 0b000100,
            InstructionKind::Jmp        => 0b000110,
            InstructionKind::JmpIf      => 0b001000,
            InstructionKind::Call       => 0b001010,
            InstructionKind::Ret        => 0b001100,
            InstructionKind::Halt       => 0b111110,
            InstructionKind::Nop        => 0b000000,
        }
    }
}

#[derive(Debug)]
pub enum Parameter {
    BeltIndex(BeltIdx),
    Immediate(Immediate),
}
