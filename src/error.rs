use colored::{Colorize, ColoredString};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum BasmError {
    NonAsciiInput,
    InvalidLabel,
    ParameterNbMismatch,
    InvalidNumberRepr,
    OutOfBoundBeltIdx,
    OutOfBoundImmediate,
    NegativePattern,
    InvalidInstruction,
    CompilationFailed,
}

impl fmt::Display for BasmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BasmError::NonAsciiInput => write!(f, "Non ascii character detected (sorry you have to find it yourself)"),
            BasmError::InvalidLabel => write!(f, "Undefined label called"),
            BasmError::ParameterNbMismatch => write!(f, "Parameter number mismatch"),
            BasmError::InvalidNumberRepr => write!(f, "Invalid number representation"),
            BasmError::OutOfBoundBeltIdx => write!(f, "Out of bound belt index ([0, 31])"),
            BasmError::OutOfBoundImmediate => write!(f, "Out of bound immediate ([-33554432, 33554431])"),
            BasmError::NegativePattern => write!(f, "Can't use negative sign before binary/hex bits pattern"),
            BasmError::InvalidInstruction => write!(f, "Invalid instruction"),
            BasmError::CompilationFailed => write!(f, "--- Compilation failed ---"),
        }
    }
}

impl BasmError {
    pub fn emit(&self, line_nb: usize, source_line: &str) {
        let error_prefix: ColoredString = "-- Error --".bold().red();
        if *self == BasmError::CompilationFailed {
            eprintln!("{}", format!("{}", self).on_red());
        } else {
            eprintln!("{}", error_prefix);
            eprintln!("{} {}", format!("{} |", line_nb).blue(), source_line);
            eprintln!("--> {}", self);
        }
    }
}
