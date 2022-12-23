use core;


pub struct WasmPrinter {
    buffer: String,
}

impl WasmPrinter {
    pub fn new() -> Self {
        Self { buffer: String::new() }
    }

    fn to_string(&self) -> String {
        self.buffer.clone()
    }
}

impl std::io::Write for WasmPrinter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let write_str = match std::str::from_utf8(buf) {
            Ok(v) => v,
            Err(_) => panic!("Invalid utf-8 sequence."),
        };
        self.buffer.push_str(write_str);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.clear();
        Ok(())
    }
}


fn main() {
    let input = "print 1 + 1;".to_string();
    let mut output = WasmPrinter::new();
    core::evaluate(input, &mut output);
    print!("{}", output.to_string());
}
