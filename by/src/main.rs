use compiler;
use std::io;

fn main() {
    let input = String::from("print \"hello, world\";");
    compiler::evaluate(input, &mut io::stdout());
}
