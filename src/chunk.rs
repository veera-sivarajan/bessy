enum OpCode {
    Add,
    Min,
    Sub,
    Mul,
    Div,
    Ret,
    Num,
}

enum Value {
    Number(f64),
}

pub struct Chunk {
    code: Vec<OpCode>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }
}
