use std::collections::HashMap;

use crate::ast::{Program, InstructionKind, Instruction};
use crate::error::BasmError;

pub fn parse<'a>(input: &'a str) -> Result<Program<'a>, BasmError> {
    if !input.is_ascii() {
        return Err(BasmError::NonAsciiInput);
    }

    let mut labels: HashMap<&'a str, usize> = HashMap::new();
    let mut remaining_lines: Vec<&'a str> = Vec::new();

    separation_pass(input, &mut labels, &mut remaining_lines);

    Ok(Program::new(Vec::new()))
}

fn separation_pass<'a>(input: &'a str, labels: &mut HashMap<&'a str, usize>, remaining_lines: &mut Vec<&'a str>) {
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
