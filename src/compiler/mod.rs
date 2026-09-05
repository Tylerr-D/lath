use crate::ast::Node;
use crate::ast::parse;

pub mod interpreter;
pub mod bytecode;
pub mod vm;


pub trait Compile {
    type Output;

    fn from_ast(ast: Vec<Node>) -> Self::Output;

    fn from_source(source: &str) -> Self::Output {
        println!("Compling:{}",source);
        let ast: Vec<Node> = parse(source).unwrap();
        println!("{:?}",ast);
        Self::from_ast(ast)
    }
}