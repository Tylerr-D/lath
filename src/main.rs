mod ast;
mod compiler;
mod parser;

use compiler::vm::opcode::Interpreter;
use compiler::vm::VM;
use compiler::Compile;
use std::io::{self, Write};

fn main() {
    println!("calculate prompts");

    loop {
        print!(">>");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed");

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "exit" || input == "quit" {
            break;
        }

        let byte_code = Interpreter::from_source(input);
        println!("byte code {:?}", byte_code);

        let mut vm = VM::new(byte_code);
        vm.run();

        println!("{:?}", vm.pop_last());
    }
}
