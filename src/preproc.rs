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
            Macro,
            extract_number,
            is_valid_identifier,
        },
        error::BasmError,
};

#[derive(Debug)]
pub struct PreProc<'a> {
    preprocessed_lines: Vec<AnnotatedLine<'a>>,
    labels: HashMap<&'a str, usize>,
    constants: HashMap<&'a str, &'a str>,
    macros: HashMap<&'a str, Macro<'a>>,
    current_line: usize,
    location_counter: usize,
    failed: bool,
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
            failed: false,
        }
    }

    pub fn take_labels(&mut self) -> HashMap<&'a str, usize> {
        mem::take(&mut self.labels)
    }

    pub fn preprocess(
        &mut self,
        input: &'a str
    ) -> Result<Vec<AnnotatedLine<'a>>, BasmError> {
        let mut lines: Vec<&str> = input.lines().map(|l| remove_comment(l.trim())).collect();

        while self.current_line < lines.len() {
            let before = self.current_line;
            if let Err(e) = self.process_line(&mut lines) {
                self.failed = true;
                let annotated = AnnotatedLine::new(
                    0,
                    SourceKind::SourceLine(before),
                    Line::new(Vec::new()),
                );
                e.emit(input, annotated);
                if self.current_line == before {
                    self.current_line += 1;
                }
            }
        }

        if self.failed {
            Err(BasmError::CompilationFailed)
        } else {
            Ok(mem::take(&mut self.preprocessed_lines))
        }
    }

    fn process_line(&mut self, lines: &mut Vec<&'a str>) -> Result<(), BasmError> {
        let line = lines[self.current_line];
        let cleaned_line = self.handle_label(line)?;
        if cleaned_line.is_empty() {
            self.current_line += 1;
            return Ok(());
        }

        let mut words = cleaned_line.split_whitespace();
        if let Some(word) = words.next() {
            if is_directive(word) {
                self.handle_directive(lines, &mut words, word)?;
            } else if self.macros.contains_key(word) {
                self.handle_macro_call(&mut words, word)?;
            } else {
                self.handle_instruction(&mut words, word)?;
            }
        }
        Ok(())
    }

    fn handle_label(&mut self, line: &'a str) -> Result<&'a str, BasmError> {
        match line.split_once(':') {
            Some((label, remaining)) => {
                let label = label.trim();
                if !label.is_empty() {
                    if is_instruction_name(label) || !is_valid_identifier(label) {
                        return Err(BasmError::InvalidLabelDefinition);
                    }
                    self.labels.insert(label, self.location_counter);
                }
                Ok(remaining.trim())
            }
            None => Ok(line),
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
        lines: &mut Vec<&'a str>,
        words: &mut SplitWhitespace<'a>,
        word: &'a str
    ) -> Result<(), BasmError> {
        match word {
            ".eq" => self.handle_constant(words)?,
            ".space" => self.handle_space(words)?,
            ".macro" => self.handle_macro(lines, words)?,
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
                if is_instruction_name(replacement) {
                    return Err(BasmError::InvalidLabelDefinition);
                }
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

    fn collect_macro_lines(
        &mut self,
        lines: &mut Vec<&'a str>,
    ) -> Result<Vec<(usize, &'a str)>, BasmError> {
        let mut macro_lines = Vec::new();

        while !lines[self.current_line].trim().starts_with(".endmacro") {
            macro_lines.push((self.current_line, lines[self.current_line]));
            self.current_line += 1;
            if self.current_line >= lines.len() {
                return Err(BasmError::NeverEndingMacro);
            }
        }

        Ok(macro_lines)
    }

    fn handle_macro(
        &mut self,
        lines: &mut Vec<&'a str>,
        words: &mut SplitWhitespace<'a>,
    ) -> Result<(), BasmError> {
        let macro_name = words
            .next()
            .ok_or(BasmError::EmptyMacroDefinition)?;

        let nb_parameter_str = words
            .next()
            .ok_or(BasmError::InvalidNumberRepr)?;

        let nb_parameter = parse_parameter_count(nb_parameter_str)?;

        self.current_line += 1;

        let macro_lines = self.collect_macro_lines(lines)?;

        self.current_line += 1;

        let macro_def = Macro::new(nb_parameter, macro_lines);

        self.macros.insert(macro_name, macro_def);

        Ok(())
    }

    fn handle_macro_call(
        &mut self,
        words: &mut SplitWhitespace<'a>,
        macro_name: &'a str,
    ) -> Result<(), BasmError> {
        let macro_def = self.macros.get(macro_name)
            .expect("Called with a defined macro");
        let nb_parameter = macro_def.nb_parameter() as usize;

        let mut args: Vec<&'a str> = Vec::with_capacity(nb_parameter);
        for w in words {
            if let Some(replacement) = self.constants.get(w) {
                args.push(replacement);
            } else {
                args.push(w);
            }
        }
        if args.len() != nb_parameter {
            return Err(BasmError::ParameterNbMismatch);
        }

        let content: Vec<(usize, &'a str)> = macro_def.content().to_vec();

        for (orig_line_nb, body_line) in content {
            let mut body_words = body_line.split_whitespace();
            if let Some(first) = body_words.next() {
                let mut line_words = vec![resolve_word(first, &args, &self.constants)];
                for w in body_words {
                    line_words.push(resolve_word(w, &args, &self.constants));
                }
                let a = AnnotatedLine::new(
                    self.location_counter,
                    SourceKind::MacroExpansion(macro_name, orig_line_nb),
                    Line::new(line_words),
                );
                self.location_counter += 4;
                self.preprocessed_lines.push(a);
            }
        }
        self.current_line += 1;
        Ok(())
    }
}

fn resolve_word<'a>(
    word: &'a str,
    args: &[&'a str],
    constants: &HashMap<&'a str, &'a str>,
) -> &'a str {
    if let Some(idx) = parse_macro_param_index(word) {
        if idx >= 1 && (idx as usize) <= args.len() {
            return args[idx as usize - 1];
        }
    }
    constants.get(word).copied().unwrap_or(word)
}

fn parse_macro_param_index(word: &str) -> Option<u32> {
    let rest = word.strip_prefix('$')?;
    if rest.is_empty() || !rest.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    rest.parse::<u32>().ok()
}

fn parse_parameter_count(s: &str) -> Result<u8, BasmError> {
    if s.len() > 2 {
        return Err(BasmError::ToManyParameter);
    }

    s.parse::<u8>().map_err(|_| BasmError::InvalidNumberRepr)
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

fn is_instruction_name(word: &str) -> bool {
    matches!(
        word,
        "add"       |
        "sub"       |
        "mul"       |
        "div"       |
        "and"       |
        "or"        |
        "xor"       |
        "sll"       |
        "srl"       |
        "sra"       |
        "eq"        |
        "lt"        |
        "put"       |
        "pick"      |
        "load8"     |
        "store8"    |
        "load16"    |
        "store16"   |
        "load32"    |
        "store32"   |
        "immh"      |
        "imml"      |
        "jmp"       |
        "jmpif"     |
        "call"      |
        "ret"       |
        "halt"      |
        "nop"
    )
}
