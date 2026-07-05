use std::{
    collections::HashMap,
    str::SplitWhitespace
};

use crate::{
        utils::{
            Line,
            AnnotatedLine,
            SourceKind,
            extract_number,
        },
        error::BasmError,
};

#[derive(Debug)]
pub struct PreProc<'a> {
    preprocessed_lines: Vec<AnnotatedLine<'a>>,
    labels: HashMap<&'a str, i32>,
    constants: HashMap<&'a str, &'a str>,
    macros: HashMap<&'a str, Vec<Line<'a>>>,
    current_line: usize,
    location_counter: usize,
}

impl<'a> PreProc<'a> {
    pub fn new() -> Self {
        Self {
            preprocessed_lines: Vec::new(),
            labels: HashMap::new(),
            constants: HashMap::new(),
            macros: HashMap::new(),
            current_line: 0,
            location_counter: 0,
        }
    }

    pub fn preprocess(&mut self, input: &'a str) -> Result<Vec<AnnotatedLine<'a>>, BasmError> {
        let cleaned_lines = self.separation_pass(input);

        while self.current_line < cleaned_lines.len() {
            let mut words = cleaned_lines[self.current_line].split_whitespace();
            if let Some(word) = words.next() {
                if is_directive(word) {
                    self.handle_directive(&mut words, word)?;
                } else {
                    self.handle_instruction(&mut words, word)?;
                }
            }
        }

        Ok(self.preprocessed_lines.clone())
    }

    fn handle_instruction(
        &mut self,
        words: &mut SplitWhitespace<'a>,
        word: &'a str
    ) -> Result<(), BasmError> {
        let mut line_words = vec![word];
        for w in words.into_iter() {
            if let Some(replacement) = self.constants.get(w) {
                line_words.push(replacement);
            } else {
                line_words.push(w);
            }
        }
        let a = AnnotatedLine::new(self.location_counter, SourceKind::SourceLine(self.current_line), Line::new(line_words));
        self.location_counter += 4; // 4 bytes instructions
        self.preprocessed_lines.push(a);
        self.current_line += 1;
        Ok(())
    }

    fn handle_directive(
        &mut self,
        words: &mut SplitWhitespace<'a>,
        word: &'a str
    ) -> Result<(), BasmError> {
        match word {
            ".eq" => self.handle_constant(words)?,
            ".space" => self.handle_space(words)?,
            _ => (),
        }
        self.current_line += 1;
        Ok(())
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
        words: &mut SplitWhitespace<'a>
    ) -> Result<(), BasmError> {
        if let Some(skip_byte) = words.next() {
            let nb_skip: usize;
            if let Some(nb_str) = self.constants.get(skip_byte) { // using a constant as parameter
                nb_skip = extract_number(nb_str)? as usize;
            } else {
                nb_skip = extract_number(skip_byte)? as usize;
            }
            self.location_counter += nb_skip;
        }
        if words.count() > 0 {
            return Err(BasmError::ParameterNbMismatch);
        }
        Ok(())
    }

    fn separation_pass(&mut self, input: &'a str) -> Vec<&'a str> {
        let mut instruction_counter = 0;
        let mut lines = input.lines();

        let mut out_lines = Vec::new();

        while let Some(line) = lines.next() {

            let trimmed_line = line.trim();
            let split = trimmed_line.split_once(':');
            match split {
                Some((label, remaining)) => {
                    self.labels.insert(label, instruction_counter);
                    if !remaining.is_empty() {
                        instruction_counter += 1;
                        out_lines.push(remove_comment(remaining));
                    }
                },
                None => {
                    if !trimmed_line.is_empty() {
                        instruction_counter += 1;
                        out_lines.push(remove_comment(trimmed_line));
                    }
                }
            }
        }
        out_lines
    }
}

fn remove_comment<'a>(line: &'a str) -> &'a str {
    match line.split_once(';') {
        None => line,
        Some((start, _)) => start,
    }
}

fn is_directive(word: &str) -> bool {
    matches!(
        word,
        ".eq"       |
        ".space"    |
        ".macro"
        )
}
