// Interned string type implemented based on  https://matklad.github.io/2020/03/22/fast-simple-rust-interner.html
use std::collections::HashMap;

#[derive(Default)]
pub struct IStrings {
    map: HashMap<String, usize>,
    list: Vec<String>,
}

impl IStrings {
    pub fn intern(&mut self, name: &str) -> usize {
        if let Some(&index) = self.map.get(name) {  // using hashmap because O(1) lookup in theory. Not sure about Rust's complexity with all its safety checks
            index
        } else {
            let index = self.map.len();
            self.map.insert(name.to_owned(), index);
            self.list.push(name.to_owned());
            index
        }
    }

    pub fn lookup(&self, index: usize) -> &str {
        self.list[index].as_str()
    }
}
    
