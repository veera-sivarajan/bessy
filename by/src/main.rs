use compiler;

fn main() {
    let input = String::from("print 1 + 1;");
    println!("Output: {}", compiler::evaluate(input));
}
