use crate::ast::parse;
use crate::ast::Node;

pub mod interpreter;
pub mod vm;

pub trait Compile {
    type Output;

    fn from_ast(ast: Vec<Node>) -> Self::Output;

    fn from_source(source: &str) -> Self::Output {
        println!("Compling:{}", source);
        let ast: Vec<Node> = parse(source).unwrap();
        println!("{:?}", ast);
        Self::from_ast(ast)
    }
}
