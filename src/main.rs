mod ast;
mod parser;
mod compiler;

use std::io::{self,Write};
use compiler::Compile;
use compiler::interpreter::Interpreter;

fn main() {

    println!("calculate prompts");

    loop {
        print!(">>");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed");

        let input = input.trim();

        if input.is_empty(){
            continue;
        }

        if input == "exit" || input == "quit" {
            break;
        }


          match Interpreter::from_source(input) {
            Ok(result) => println!("{}", result),
            Err(e) => println!("Error: {}", e),
        }
    }

}