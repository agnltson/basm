use std::collections::VecDeque;
use crate::error::BasmError;

#[derive(Debug, Clone)]
pub struct Line<'a> {
    pub words: VecDeque<&'a str>, // a line is a vec of word
}

impl<'a> Line<'a> {
    pub fn new(words_vec: Vec<&'a str>) -> Self {
        Self {
            words: words_vec.into(),
        }
    }
}

impl<'a> Iterator for Line<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.words.pop_front()
    }
}

#[derive(Debug, Clone)]
pub enum SourceKind<'a> {
    MacroExpansion(&'a str, usize), // macro name and line in file
    SourceLine(usize),
}

#[derive(Debug, Clone)]
pub struct AnnotatedLine<'a> {
    pub location: usize,
    pub source_kind: SourceKind<'a>,
    pub line: Line<'a>,
}

impl<'a> AnnotatedLine<'a> {
    pub fn new(location: usize, source_kind: SourceKind<'a>, line: Line<'a>) -> Self {
        Self {
            location,
            source_kind,
            line,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Macro<'a> {
    nb_parameter: u8,
    content: Vec<(usize, &'a str)>, // (source line nb, line content)
}

impl<'a> Macro<'a> {
    pub fn new(nb_parameter: u8, content: Vec<(usize, &'a str)>) -> Self {
        Self {
            nb_parameter,
            content,
        }
    }

    pub fn nb_parameter(&self) -> u8 {
        self.nb_parameter
    }

    pub fn content(&self) -> &[(usize, &'a str)] {
        &self.content
    }
}

pub fn is_valid_identifier(ident: &str) -> bool {
    let mut chars = ident.chars();

    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn extract_number(val: &str) -> Result<i32, BasmError> {
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
