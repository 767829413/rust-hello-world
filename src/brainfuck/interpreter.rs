use std::io::{Read, Write};

use super::opcode::{Code, Opcode};
pub struct Interpreter {
    // 类似无限展开的纸带
    stack: Vec<u8>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { stack: vec![0; 1] }
    }

    pub fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let code = Code::from(data)?;
        let code_len = code.len();
        let mut pc: usize = 0; //程序运行计数,定位执行位置
        let mut sp: usize = 0; //stack 的位置

        loop {
            if pc >= code_len {
                break;
            }
            match code.instrs[pc] {
                Opcode::SHR => {
                    sp += 1;
                    if sp == self.stack.len() {
                        self.stack.push(0);
                    }
                }
                Opcode::SHL => {
                    if sp > 0 {
                        sp -= 1;
                    }
                }
                Opcode::ADD => {
                    self.stack[sp] = self.stack[sp].overflowing_add(1).0;
                }
                Opcode::SUB => {
                    self.stack[sp] = self.stack[sp].overflowing_sub(1).0;
                }
                Opcode::PUTCHR => {
                    std::io::stdout().write_all(&[self.stack[sp]])?;
                }
                Opcode::GETCHR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[sp] = buf[0];
                }
                Opcode::LB => {
                    if self.stack[sp] == 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
                Opcode::RB => {
                    if self.stack[sp] != 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
            }
            pc += 1;
        }
        Ok(())
    }
}
