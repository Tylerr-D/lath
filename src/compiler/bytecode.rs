use crate::ast::{Node, Operator};

#[derive(Debug, PartialEq)]
pub enum OpCode {
    Constant(i32),
    Add,
    Subtract,
    Negate,
}

pub fn compile(ast: &[Node]) -> Vec<OpCode> {
    let mut code = Vec::new();

    for node in ast {
        compile_node(node, &mut code);
        
    }

}