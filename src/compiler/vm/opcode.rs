use crate::ast::Node;
use crate::compiler::Compile;

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

pub struct Bytecode {
    pub instructions: Vec<u8>,
    pub constants: Vec<Node>,
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
     interpreter.add_instruction(OpCode::OpPop);
}

        interpreter.bytecode

}
}