use compiler;

fn main() {
    let input = String::from("var a = 10; print a + 2;");
    println!("Output: {}", compiler::evaluate(input));
}
