mod parser;
mod ast;
mod error;
mod numerics;

fn main() {
    let input = "
        main:
            add -1 31
            immh 0b2
            imml 0xM

            halt
            jmp main
        ";
    let res = parser::parse(input);
    match res {
        Ok(prog) => println!("program: {:?}", prog),
        Err(e) => e.emit(0, ""),
    }
}
