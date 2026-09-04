pub enum OpCode {
    OpConstant(u16),
    OpPOp,
    OpAdd,
    OpSub,
    OpPlus,
    OpMinus,
}

OpCode::OpConstant(arg) => make_three_bytes_op(0x01, arg),
OpCode::OpPop => vec![0x02],
OpCode::OpAdd => vec![0x03],
OpCode::OpSub => vec![0x04],
OpCode::OpPlus => vec![0x0A],
