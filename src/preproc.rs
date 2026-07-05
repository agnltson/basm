use std::{
    collections::HashMap,
    str::SplitWhitespace,
    mem,
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
    labels: HashMap<&'a str, usize>,
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

    pub fn take_labels(&mut self) -> HashMap<&'a str, usize> {
        mem::take(&mut self.labels)
    }

    pub fn preprocess(
        &mut self,
        input: &'a str
    ) -> Result<Vec<AnnotatedLine<'a>>, BasmError> {
        let mut lines = input.lines().map(|l| remove_comment(l.trim()));

        while let Some(line) = lines.next() {
            let cleaned_line = self.handle_label(line);
            if cleaned_line.is_empty() {
                continue;
            }

            let mut words = cleaned_line.split_whitespace();
            if let Some(word) = words.next() {
                if is_directive(word) {
                    self.handle_directive(&mut words, word)?;
                } else {
                    self.handle_instruction(&mut words, word)?;
                }
            }
        }
        Ok(mem::take(&mut self.preprocessed_lines))
    }

    fn handle_label(&mut self, line: &'a str) -> &'a str {
        match line.split_once(':') {
            Some((label, remaining)) => {
                let label = label.trim();
                if !label.is_empty() {
                    self.labels.insert(label, self.location_counter);
                }
                remaining.trim()
            }
            None => line,
        }
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
