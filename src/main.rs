mod parser;
mod ast;
mod error;
mod numerics;

fn main() {
    let input = "
        main:
            add 23, 1

            halt
        ";
    let res = parser::parse(input);
    match res {
        Ok(prog) => println!("program: {:?}", prog),
        Err(_) => (),
    }
}
