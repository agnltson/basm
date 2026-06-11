use std::collections::HashMap;
use std::str::SplitWhitespace;

use crate::ast::{Program, InstructionKind, Instruction, Parameter};
use crate::error::BasmError;
use crate::numerics::{u5, i26};

pub fn parse(input: & str) -> Result<Program, BasmError> {
    if !input.is_ascii() {
        return Err(BasmError::NonAsciiInput);
    }

    let mut labels: HashMap<&str, i32> = HashMap::new();
    let mut remaining_lines: Vec<&str> = Vec::new();

    separation_pass(input, &mut labels, &mut remaining_lines);

    let mut instruction_counter = 0;

    let mut instructions: Vec<Instruction> = Vec::new();

    for mut line in remaining_lines.iter().map(|l| l.split_whitespace()).into_iter() {
        if let Some(word) = line.next() {
            let kind: InstructionKind = InstructionKind::get_instruction_kind(word)?;
            match collect_parameters(&labels, instruction_counter, &kind, &mut line) {
                Ok(parameters) => {
                    if parameters.len() == kind.nb_parameter() {
                        instructions.push(Instruction::new(kind, parameters));
                    } else {
                        return Err(BasmError::Default);
                    }
                },
                Err(e) => { return Err(e) },
            }
        } else {
            continue;
        }
    }
    Ok(Program::new(instructions))
}

fn separation_pass<'a>(input: &'a str, labels: &mut HashMap<&'a str, i32>, remaining_lines: &mut Vec<&'a str>) {
    let mut instruction_counter = 0;

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let trimmed_line = line.trim();
        let split = trimmed_line.split_once(':');
        match split {
            Some((label, remaining)) => {
                labels.insert(label, instruction_counter);
                if !remaining.is_empty() {
                    instruction_counter += 1;
                    remaining_lines.push(remaining);
                }
            },
            None => {
                if !trimmed_line.is_empty() {
                    instruction_counter += 1;
                    remaining_lines.push(trimmed_line);
                }
            }
        }
    }
}

fn collect_parameters(labels: &HashMap<&str, i32>, instruction_counter: i32, kind: &InstructionKind, line: &mut SplitWhitespace) -> Result<Vec<Parameter>, BasmError> {
    if kind.is_type0() {
        collect_i26_parameters(labels, instruction_counter, line)
    } else {
        collect_u5_parameters(line)
    }
}

fn collect_i26_parameters(labels: &HashMap<&str, i32>, instruction_counter: i32, line: &mut SplitWhitespace) -> Result<Vec<Parameter>, BasmError> {
    let mut parameters = Vec::new();
    while let Some(param) = line.next() {
        if is_number(param) {
            let val = extract_number(param)?;
            parameters.push(Parameter::Immediate(i26(val)));
        } else {
            if let Some(addr) = labels.get(param) {
                let offset = addr - instruction_counter - 1;
                parameters.push(Parameter::Immediate(i26(offset)));
            } else {
                return Err(BasmError::Default);
            }
        }
    }
    Ok(parameters)
}

fn collect_u5_parameters(line: &mut SplitWhitespace) -> Result<Vec<Parameter>, BasmError> {
    let mut parameters = Vec::new();
    while let Some(param) = line.next() {
        if is_number(param) {
            let val = extract_number(param)?;
            parameters.push(Parameter::BeltIndex(u5(val as u8)));
        } else {
            return Err(BasmError::Default);
        }
    }
    Ok(parameters)
}

fn is_number(val: &str) -> bool {
    if let Some(first) = val.chars().next() {
        matches!(first, '0'..'9' | '-')
    } else {
        false
    }
}

// hexa/binary representation are parsed as bits pattern.
// For immediate we are using two's complement (eg 0xFFFFFFFF -> -1).
// For Belt index we use regular unsigned representation (ex 0b11111 -> 31).
// in decimal the value mean the value (eg -1 = -1).
fn extract_number(val: &str) -> Result<i32, BasmError> {
    let chars: Vec<char> = val.chars().collect();
    let radix;
    let start;

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

    let n = (chars.len() - start) as u32;
    let mut result = 0u32;

    for i in start..chars.len() {
        match chars[i].to_digit(radix) {
            None => return Err(BasmError::Default),
            Some(d) => {
                result = result
                    .checked_add(d * radix.pow(n - (i + 1 - start) as u32))
                    .ok_or(BasmError::Default)?;
            }
        }
    }

    if radix == 10 {
        if result > i32::MAX as u32 {
            // overflow
            return Err(BasmError::Default);
        }
        Ok(result as i32)
    } else {
        Ok(result as i32)
    }
}

fn is_binary(chars: &Vec<char>) -> bool {
    chars[0] == '0' && chars[1] == 'b'
}

fn is_hexa(chars: &Vec<char>) -> bool {
    chars[0] == '0' && (chars[1] == 'x' || chars[1] == 'X')
}
