// these are my personal notes pls dont mind them and umm not ai 
// ok so here we gonna first take bytecode
// bytecodes are instructions
// then we got our big ahh VM
// VM reads the byte and understands what it means with the help of our opcode.rs that i js wrote
// now the vm gonna through everything to the stack
// does the operation required
// moves to the next byte does the same shit again
// lets fucking go

const STACK_SIZE: usize = 512;

pub struct VM {
    bytecode:Bytecode,
    stack: [Node; STACK_SIZE],
    stack_ptr: usize,
}

pub fn run(&mut self) {
    let mut ip = 0;
    while ip < self.bytecode.instructions.len(){
        let instr_addr = ip;
        ip +=1;
    

    match self.bytecode.instructions[instr_addr]{
        0x01 => {

            // - OpConst
            let const_idx = convert_two_u8s_to_usize_hehehehe(
                self.bytecode.instructions[ip],
                self.bytecode.instructions[ip + 1],
            );
            ip+=2;
            self.push(self.bytecode.constants[const_idx].clone());
        }

        0x02 => {
            // umm this is OpPop 
                                self.pop();

        }

        0x03 => {
            // its OpAdd
            match (self.pop(), self.pop()) {
                (Node::Int(rhs),Node::Int(lhs)) => self.push(Node::Int(lhs + rhs)),
                _=> panic!("unknown types to OpAdd"),
            }
        }

                0x04 => {
            // um OpSub
            match (self.pop(), self.pop()) {
                (Node::Int(rhs),Node::Int(lhs)) => self.push(Node::Int(lhs - rhs)),
                _=> panic!("unknown types to OpSub"),
            }
        }

        0x0A => {
            // OpPlus

            match self.pop(){
                Node::Int(num) => self.push(Node::Int(num)),
                _=> panic!("unknown type to OpPlus"),
            }
        }

        0x0B => {
            // the last one - OpMinus

            match self.pop(){
                Node::Int(num) => self.push(Node::Int(-num)),
                _ => panic!("unknown type to OpMinus"),

            }
        }

        _ => panic!("some unknown instruction appeared noooo"),

    }
}
}

pub fn push(&mut self, node: Node) {
    self.stack[self.stack_ptr] = node;
    self.stack_ptr += 1;
}

pub fn pop (&mut self) -> Node {
            let node = self.stack[self.stack_ptr - 1].clone();
        self.stack_ptr -= 1;
        node
}