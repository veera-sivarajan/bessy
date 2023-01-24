use std::io::Write;

fn get_input() -> String {
    let mut input = String::new();
    input.clear();
    print!("rena$ ");
    let _ = std::io::stdout().flush();
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    let _ = input.pop();

    input
}

fn main() {
    // loop {
    //     let input = get_input();
    //     core::evaluate(&input);
    // }
    let input = include_str!("../../test/hello.lox");
    core::evaluate(&input);
}
