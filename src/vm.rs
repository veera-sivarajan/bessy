use crate::chunk::Chunk;

pub struct VM<'c> {
    chunk: &'c Chunk,
    ip: usize,
}

impl<'c> VM<'c> {
    pub fn new(chunk: &'c Chunk) -> Self {
        VM { chunk, ip: 0 }
    }
}
        
