use colored::{Colorize, ColoredString};

#[derive(Debug, PartialEq)]
pub enum BasmError {
    NonAsciiInput,
    InvalidLabel,
    ParameterNbMismatch,
    InvalidNumberRepr,
    OutOfBoundU5,
    OutOfBoundI26,
    NegativePattern,
    CompilationFailed,
}

impl BasmError {
    pub fn to_format(&self) -> &str {
        match self {
            BasmError::NonAsciiInput => "Non ascii character detected (sorry you have to find it yourself)",
            BasmError::InvalidLabel => "Undefined label called",
            BasmError::ParameterNbMismatch => "Parameter number mismatch",
            BasmError::InvalidNumberRepr => "Invalid number representation",
            BasmError::OutOfBoundU5 => "Out of bound u5 ([0, 31])",
            BasmError::OutOfBoundI26 => "Out of bound i26 ([-33554432, 33554431])",
            BasmError::NegativePattern => "Can't use negative sign before binary/hex bits pattern",
            BasmError::CompilationFailed => "--- Compilation failed ---",
        }
    }

    pub fn emit(&self, line_nb: usize, source_line: &str) {
        let error_prefix: ColoredString = "-- Error --".bold().red();
        if *self == BasmError::CompilationFailed {
            eprintln!("{}", self.to_format().on_red());
        } else {
            eprintln!("{}", error_prefix);
            eprintln!("{} {}", format!("{} |", line_nb).blue(), source_line);
            eprintln!("--> {}", self.to_format());
        }
    }
}
