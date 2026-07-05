use std::{
    mem,
    collections::HashMap,
};
use crate::{
        ast::{
            Program,
            InstructionKind,
            Instruction,
            Parameter
        },
        error::{
            BasmError
        },
        numerics::{
            BeltIdx,
            Immediate,
        },
        preproc::{
            PreProc,
        },
        utils::{
            AnnotatedLine,
            Line,
            extract_number,
        },
};

#[derive(Debug)]
pub struct Parser<'a> {
    labels: HashMap<&'a str, usize>,
    instructions: Vec<Instruction>,
    location_counter: usize,
    failed: bool,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Self {
            labels: HashMap::new(),
            instructions: Vec::new(),
            location_counter: 0,
            failed: false,
        }
    }

    pub fn parse(&mut self, input: &'a str) -> Result<Program, BasmError> {
        if !input.is_ascii() {
            return Err(BasmError::NonAsciiInput);
        }
        let mut preproc = PreProc::new();
        let preprocessed_lines = preproc.preprocess(input)?;
        self.labels = preproc.take_labels();
        for line in preprocessed_lines.into_iter() {
            self.location_counter += line.location;
            match self.handle_instruction(&line) {
                    Err(e) => {
                        self.failed = true;
                        e.emit(input, line);
                        continue;
                    },
                    _ => (),
            }
        }
        if self.failed {
            Err(BasmError::CompilationFailed)
        } else {
            Ok(Program::new(mem::take(&mut self.instructions)))
        }
    }

    fn handle_instruction(
        &mut self,
        ann_line: &AnnotatedLine<'a>,
    ) -> Result<(), BasmError> {
        let mut words = ann_line.line.clone().into_iter();
        if let Some(inst_name) = words.next() {
            let kind: InstructionKind = InstructionKind::get_instruction_kind(inst_name)?;
            let parameters = self.collect_parameters(&kind, &mut words)?;

            if parameters.len() == kind.nb_parameter() {
                self.instructions.push(Instruction::new(ann_line.location, kind, parameters));
                Ok(())
            } else {
                Err(BasmError::ParameterNbMismatch)
            }
        } else {
            Ok(())
        }
    }

    fn collect_parameters(
        &mut self,
        kind: &InstructionKind,
        words: &mut Line<'a>,
    ) -> Result<Vec<Parameter>, BasmError> {
        if kind.is_type0() {
            self.collect_immediate_parameters(words)
        } else {
            self.collect_belt_idx_parameters(words)
        }
    }

    fn collect_immediate_parameters(
        &mut self,
        words: &mut Line<'a>,
        ) -> Result<Vec<Parameter>, BasmError> {
        let mut parameters = Vec::new();
        while let Some(param) = words.next() {
            if is_number(param) {
                let val = extract_immediate(param)?;
                parameters.push(Parameter::Immediate(val));
            } else if let Some(addr) = self.labels.get(param) {
                    parameters.push(Parameter::Immediate(Immediate(*addr as i32)));
            } else {
                return Err(BasmError::InvalidParameter);
            }
        }
        Ok(parameters)
    }

    fn collect_belt_idx_parameters(
        &mut self,
        words: &mut Line<'a>,
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
