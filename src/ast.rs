#[derive(Debug)]
pub struct Program<'a> {
    instructions: Vec<Instruction<'a>>,
}

impl<'a> Program<'a> {
    pub fn new(instructions: Vec<Instruction<'a>>) -> Self {
        Self {
            instructions,
        }
    }
}

#[derive(Debug)]
pub struct Instruction<'a> {
    kind: InstructionKind,
    parameter: Parameter<'a>,
}

impl<'a> Instruction<'a> {
    pub fn new(kind: InstructionKind, parameter: Parameter<'a>) -> Self {
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

    pub fn nb_parameter(&self) -> u8 {
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
}

impl From<&str> for InstructionKind {
    fn from(name: &str) -> Self {
        match name {
            "add" => InstructionKind::Add,
            "sub" => InstructionKind::Sub,
            "mul" => InstructionKind::Mul,
            "div" => InstructionKind::Div,
            _ => InstructionKind::Nop,
        }
    }
}

#[derive(Debug)]
pub enum Parameter<'a> {
    None,
    Number(i32),
    TwoNumber(i32, i32),
    Label(&'a str),
}
