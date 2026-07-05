mod parser;
mod ast;
mod error;
mod numerics;
mod preproc;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input> <output>", args[0]);
        std::process::exit(1);
    }

    let input = std::fs::read_to_string(&args[1]).unwrap_or_else(|e| {
        eprintln!("Error reading '{}': {}", args[1], e);
        std::process::exit(1);
    });

    let mut parser = parser::Parser::new();
    let res = parser.parse(&input);
    match res {
        Ok(prog) => {
            let bytes: Vec<u8> = prog.into();
            std::fs::write(&args[2], bytes).unwrap_or_else(|e| {
                eprintln!("Error writing '{}': {}", args[2], e);
                std::process::exit(1);
            });
        },
        Err(e) => e.emit("", utils::AnnotatedLine::new(0, utils::SourceKind::SourceLine(0), utils::Line::new(Vec::new()))),
    }
}
