/*
字符	含义
>	指针加一
<	指针减一
+	指针所指字节的值加一
-	指针所指字节的值减一
.	输出指针所指字节内容（ASCII码）
,	向指针所指的字节输入内容（ASCII码）
[	若指针所指字节的值为零，则向后跳转，跳转到其对应的]的下一个指令处
]	若指针所指字节的值不为零，则向前跳转，跳转到其对应的[的下一个指令处
*/

// 十六进制ASCII码
#[derive(Debug, PartialEq)]
pub enum Opcode {
    SHR = 0x3E,    // >
    SHL = 0x3C,    // <
    ADD = 0x2B,    // +
    SUB = 0x2D,    // -
    PUTCHR = 0x2E, // .
    GETCHR = 0x2C, // ,
    LB = 0x5B,     // [
    RB = 0x5D,     // ]
}

impl From<u8> for Opcode {
    fn from(u: u8) -> Self {
        match u {
            0x3E => Opcode::SHR,
            0x3C => Opcode::SHL,
            0x2B => Opcode::ADD,
            0x2D => Opcode::SUB,
            0x2E => Opcode::PUTCHR,
            0x2C => Opcode::GETCHR,
            0x5B => Opcode::LB,
            0x5D => Opcode::RB,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug)]
pub struct Code {
    pub instrs: Vec<Opcode>,
    pub jtable: std::collections::HashMap<usize, usize>,
}

impl Code {
    pub fn from(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let dict = vec![
            Opcode::SHR as u8,
            Opcode::SHL as u8,
            Opcode::ADD as u8,
            Opcode::SUB as u8,
            Opcode::PUTCHR as u8,
            Opcode::GETCHR as u8,
            Opcode::LB as u8,
            Opcode::RB as u8,
        ];

        let instrs: Vec<Opcode> = data
            .iter()
            .filter(|x| dict.contains(x))
            .map(|x| Opcode::from(*x))
            .collect();

        let mut jstack: Vec<usize> = Vec::new();
        let mut jtable: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();

        // 构造一个栈来记录[]的位置
        for (i, e) in instrs.iter().enumerate() {
            if Opcode::LB == *e {
                jstack.push(i)
            }
            if Opcode::RB == *e {
                let j = jstack.pop().ok_or("pop from is empyt list")?;
                jtable.insert(i, j);
                jtable.insert(j, i);
            }
        }

        return Ok(Code {
            instrs: instrs,
            jtable: jtable,
        });
    }

    pub fn len(&self) -> usize {
        self.instrs.len()
    }
}
