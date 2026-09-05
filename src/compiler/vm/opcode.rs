use crate::compiler::Compile;
use crate::ast::{Node, Operator};


fn make_three_bytes_op(opcode: u8, arg: u16) -> Vec<u8> {
    vec![opcode, (arg & 0xFF) as u8, (arg >> 8) as u8]
}

pub enum OpCode {
    OpConstant(u16),
    OpPop,
    OpAdd,
    OpSub,
    OpPlus,
    OpMinus,
}

fn to_bytes(op: OpCode) -> Vec<u8> {

match op {
OpCode::OpConstant(arg) => make_three_bytes_op(0x01, arg),
OpCode::OpPop => vec![0x02],
OpCode::OpAdd => vec![0x03],
OpCode::OpSub => vec![0x04],
OpCode::OpPlus => vec![0x0A],
OpCode::OpMinus => vec![0x0B],
}
}

#[derive(Debug)]
pub struct Bytecode {
    pub instructions: Vec<u8>,
    pub constants: Vec<Node>,
}

impl Bytecode {

pub fn new () -> Self {
    Bytecode {
instructions: vec![], constants: vec![]}
 }
}

pub struct Interpreter {
    bytecode: Bytecode,
}

impl Compile for Interpreter {
    type Output = Bytecode; 


fn from_ast(ast: Vec<Node>) -> Self::Output {
            let mut interpreter = Interpreter {
            bytecode: Bytecode::new(),
};

for node in ast {
    println!("compliling node {:?}",node);
    interpreter.interpret_node(node);
}

        interpreter.bytecode

}
}


impl Interpreter {
    fn add_instruction(&mut self, op: OpCode) {
        self.bytecode.instructions.extend(to_bytes(op));
    }


    fn interpret_node(&mut self, node: Node) {



        match node {

            Node::Int(n) => {
                self.bytecode.constants.push(Node::Int(n));

                let idx = (self.bytecode.constants.len() - 1) as u16;

                self.add_instruction(OpCode::OpConstant(idx));
            }
            Node::UnaryExpr { op, child } => {
                self.interpret_node(*child);


                match op {
                    Operator::Plus => self.add_instruction(OpCode::OpPlus),

                Operator::Minus => self.add_instruction(OpCode::OpMinus),

                }
            }
            Node::BinaryExpr { op, lhs, rhs } => {
            self.interpret_node(*lhs);


                self.interpret_node(*rhs);

                match op {
                Operator::Plus => self.add_instruction(OpCode::OpAdd),
                    Operator::Minus => self.add_instruction(OpCode::OpSub),
                }
            }
        }



        }
    }
