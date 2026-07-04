use std::collections::{
    HashMap,
    VecDeque,
};
use std::str::SplitWhitespace;

use crate::ast::{
    Program,
    InstructionKind,
    Instruction,
    Parameter
};
use crate::error::BasmError;
use crate::numerics::{
    BeltIdx,
    Immediate
};

#[derive(Debug)]
pub struct Parser<'a> {
    lines: Vec<(usize, &'a str)>,
    current_line: usize,
    labels: HashMap<&'a str, i32>,
    constants: HashMap<&'a str, &'a str>,
    instructions: Vec<Instruction>,
    failed: bool,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            current_line: 0,
            labels: HashMap::new(),
            constants: HashMap::new(),
            instructions: Vec::new(),
            failed: false,
        }
    }

    pub fn parse(&mut self, input: &'a str) -> Result<Program, BasmError> {
        self.separation_pass(input);
        while self.current_line < self.lines.len() {
            let (line_nb, line) = self.lines[self.current_line];
            let mut words = line.split_whitespace();
            if let Some(word) = words.next() {
                let res;
                if is_directive(word) {
                    res = self.handle_directive(&mut words, word);
                } else {
                    res = self.handle_instruction(&mut words, word);
                }
                match res {
                    Err(e) => {
                        self.failed = true;
                        e.emit(line_nb, self.lines[line_nb].1);
                        continue;
                    },
                    _ => (),
                }
            }
            self.current_line += 1;
        }
        if self.failed {
            Err(BasmError::CompilationFailed)
        } else {
            Ok(Program::new(Vec::new()))
        }
    }

    fn separation_pass(&mut self, input: &'a str) {
        let mut instruction_counter = 0;
        let mut source_line_counter = 0;

        let mut lines = input.lines();

        while let Some(line) = lines.next() {
            source_line_counter += 1;

            let trimmed_line = line.trim();
            let split = trimmed_line.split_once(':');
            match split {
                Some((label, remaining)) => {
                    self.labels.insert(label, instruction_counter);
                    if !remaining.is_empty() {
                        instruction_counter += 1;
                        self.lines.push((source_line_counter, remove_comment(remaining)));
                    }
                },
                None => {
                    if !trimmed_line.is_empty() {
                        instruction_counter += 1;
                        self.lines.push((source_line_counter, remove_comment(trimmed_line)));
                    }
                }
            }
        }
    }

    fn handle_instruction(
        &mut self,
        words: &mut SplitWhitespace,
        word: &str,
        ) -> Result<(), BasmError> {

        let kind: InstructionKind = InstructionKind::get_instruction_kind(word)?;
        let parameters = self.collect_parameters(&kind, words)?;

        if parameters.len() == kind.nb_parameter() {
            self.instructions.push(Instruction::new(kind, parameters));
            Ok(())
        } else {
            Err(BasmError::ParameterNbMismatch)
        }
    }

    fn handle_directive(
        &mut self,
        words: &mut SplitWhitespace<'a>,
        directive: &str,
        ) -> Result<(), BasmError> {
        match directive {
            ".eq" => self.handle_constant(words),
            ".space" => self.handle_space(words),
            _ => todo!(),
        }
    }

    fn collect_parameters(
        &mut self,
        kind: &InstructionKind,
        words: &mut SplitWhitespace,
        ) -> Result<Vec<Parameter>, BasmError> {
        if kind.is_type0() {
            self.collect_immediate_parameters(words)
        } else if kind.is_type1() {
            self.collect_belt_idx_parameters(words)
        } else { // Internal instruction
            Ok(vec![])
        }
    }

    fn collect_immediate_parameters(
        &mut self,
        words: &mut SplitWhitespace,
        ) -> Result<Vec<Parameter>, BasmError> {
        let mut parameters = Vec::new();
        while let Some(mut param) = words.next() {
            if let Some(replacement) = self.constants.get(param) {
                param = replacement;
            }
            if is_number(param) {
                let val = extract_immediate(param)?;
                parameters.push(Parameter::Immediate(val));
            } else if let Some(addr) = self.labels.get(param) {
                    let offset = addr - (self.instructions.len() as i32) - 1;
                    parameters.push(Parameter::Immediate(Immediate(offset)));
            } else {
                return Err(BasmError::InvalidParameter);
            }
        }
        Ok(parameters)
    }

    fn collect_belt_idx_parameters(
        &mut self,
        words: &mut SplitWhitespace,
        ) -> Result<Vec<Parameter>, BasmError> {
        let mut parameters = Vec::new();
        while let Some(param) = words.next() {
            if is_number(param) {
                let val = extract_belt_idx(param)?;
                parameters.push(Parameter::BeltIndex(val));
            } else {
                return Err(BasmError::InvalidNumberRepr);
            }
        }
        Ok(parameters)
    }

    fn handle_constant(
        &mut self,
        words: &mut SplitWhitespace<'a>
        ) -> Result<(), BasmError> {
        if let Some(name) = words.next() {
            if let Some(replacement) = words.next() {
                self.constants.insert(name, replacement);
            }
        }
        if words.count() > 0 {
            return Err(BasmError::ParameterNbMismatch);
        }
        Ok(())
    }

    fn handle_space(
        &mut self,
        words: &mut SplitWhitespace
        ) -> Result<(), BasmError> {
        if let Some(skip_byte) = words.next() {
            let nb = extract_number(skip_byte)?;
            self.instructions.push(Instruction::new(InstructionKind::InternalSpace, vec![Parameter::Immediate(Immediate(nb))]));
        }
        if words.count() > 0 {
            return Err(BasmError::ParameterNbMismatch);
        }
        Ok(())
    }
}

fn remove_comment<'a>(line: &'a str) -> &'a str {
    match line.split_once(';') {
        None => line,
        Some((start, _)) => start,
    }
}

fn is_number(val: &str) -> bool {
    if let Some(first) = val.chars().next() {
        matches!(first, '0'..':' | '-') // ':' = '9'+1
    } else {
        false
    }
}

fn extract_belt_idx(val: &str) -> Result<BeltIdx, BasmError> {
    let val = extract_number(val)?;
    if val < BeltIdx::MIN.into() || val > BeltIdx::MAX.into() {
        return Err(BasmError::OutOfBoundBeltIdx);
    }
    Ok(BeltIdx(val as u8))
}

fn extract_immediate(val: &str) -> Result<Immediate, BasmError> {
    let val = extract_number(val)?;
    if val < Immediate::MIN || val > Immediate::MAX {
        return Err(BasmError::OutOfBoundImmediate);
    }
    Ok(Immediate(val))
}

// hexa/binary representation are parsed as bits pattern.
// For immediate we are using two's complement (eg 0xFFFFFFFF -> -1).
// For Belt index we use regular unsigned representation (ex 0b11111 -> 31).
// in decimal the value mean the value (eg -1 = -1).
fn extract_number(val: &str) -> Result<i32, BasmError> {
    let mut chars: Vec<char> = val.chars().collect();
    let radix;
    let start;
    let mut is_neg = false;

    if chars[0] == '-' {
        is_neg = true;
        chars = chars[1..].to_vec();
    }

    if is_binary(&chars) {
        radix = 2u32;
        start = 2;
    } else if is_hexa(&chars) {
        radix = 16u32;
        start = 2;
    } else {
        radix = 10u32;
        start = 0;
    }

    if (radix == 2 || radix == 16 ) && is_neg {
        return Err(BasmError::NegativePattern);
    }

    let n = (chars.len() - start) as u32;
    let mut result = 0u32;

    for i in start..chars.len() {
        match chars[i].to_digit(radix) {
            None => return Err(BasmError::InvalidNumberRepr),
            Some(d) => {
                result = result + d * radix.pow(n - (i + 1 - start) as u32);
            }
        }
    }
    // no overflow catch
    if is_neg {
        Ok(-(result as i32))
    } else {
        Ok(result as i32)
    }
}

fn is_binary(chars: &Vec<char>) -> bool {
    if chars.len() < 3 {
        false
    } else {
        chars[0] == '0' && chars[1] == 'b'
    }
}

fn is_hexa(chars: &Vec<char>) -> bool {
    if chars.len() < 3 {
        false
    } else {
        chars[0] == '0' && (chars[1] == 'x' || chars[1] == 'X')
    }
}

fn is_directive(word: &str) -> bool {
    matches!(
        word,
        ".eq"       |
        ".space"
        )
}
