use colored::{Colorize, ColoredString};
use std::fmt;

use crate::utils::{
    AnnotatedLine,
    SourceKind,
};

#[derive(Debug, PartialEq)]
pub enum BasmError {
    NonAsciiInput,
    InvalidParameter,
    ParameterNbMismatch,
    InvalidNumberRepr,
    OutOfBoundBeltIdx,
    OutOfBoundImmediate,
    NegativePattern,
    InvalidInstruction,
    InvalidLabelDefinition,
    ToManyParameter,
    EmptyMacroDefinition,
    NeverEndingMacro,
    CompilationFailed,
}

impl fmt::Display for BasmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BasmError::NonAsciiInput => write!(f, "Non ascii character detected (sorry you have to find it yourself)"),
            BasmError::InvalidParameter => write!(f, "Invalid parameter. If it's a label it doesn't exist."),
            BasmError::ParameterNbMismatch => write!(f, "Parameter number mismatch"),
            BasmError::InvalidNumberRepr => write!(f, "Invalid number representation"),
            BasmError::OutOfBoundBeltIdx => write!(f, "Out of bound belt index ([0, 31])"),
            BasmError::OutOfBoundImmediate => write!(f, "Out of bound immediate ([-33554432, 33554431])"),
            BasmError::NegativePattern => write!(f, "Can't use negative sign before binary/hex bits pattern"),
            BasmError::InvalidInstruction => write!(f, "Invalid instruction"),
            BasmError::InvalidLabelDefinition =>
                write!(f, "Invalid label definition. Using invalid chars in label or instruction name as label"),
            BasmError::ToManyParameter => write!(f, "Maximum macro parameter is 9"),
            BasmError::EmptyMacroDefinition => write!(f, "Trying to define a macro without a name"),
            BasmError::NeverEndingMacro => write!(f, "A macro is started but never ended"),
            BasmError::CompilationFailed => write!(f, "--- Compilation failed ---"),
        }
    }
}

impl BasmError {
    pub fn emit(&self, source: &str, annotated_lines: AnnotatedLine) {
        let error_prefix: ColoredString = "-- Error --".bold().red();
        if *self == BasmError::CompilationFailed {
            eprintln!("{}", format!("{}", self).on_red());
        } else {
            let source_lines: Vec<&str> = source.lines().into_iter().collect();
            eprintln!("{}", error_prefix);
            if source_lines.len() == 0 {
                return;
            }
            match annotated_lines.source_kind {
                SourceKind::SourceLine(line_nb) => {
                    eprintln!("{} {}", format!("{} |", line_nb).blue(), source_lines[line_nb]);
                },
                SourceKind::MacroExpansion(macro_name, line_nb) => {
                    eprintln!("In expansion of macro {}", format!("{}", macro_name).green());
                    eprintln!("{} {}", format!("{} |", line_nb).blue(), source_lines[line_nb]);
                },

            }
            eprintln!("--> {}", self);
        }
    }
}
