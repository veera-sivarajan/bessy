type Value = f64; // double precision floating point numbers

pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray { values: Vec::new() }
    }

    pub fn write_value_array(&mut self, value: Value) {
        self.values.push(value);
    }
}
