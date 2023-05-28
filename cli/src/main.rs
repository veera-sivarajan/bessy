use std::io::Write;

fn get_input() -> String {
    let mut input = String::new();
    input.clear();
    print!("bessy>> ");
    let _ = std::io::stdout().flush();
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    let _ = input.pop();

    input
}

fn main() {
    // loop {
    //     let input = get_input();
    //     match core::evaluate(&input) {
    //         Ok(()) => continue,
    //         Err(msg) => eprintln!("{msg}"),
    //     }
    // }
    let input = include_str!("../../test/hello.lox");
    match core::evaluate(input) {
        Ok(()) => {},
        Err(msg) => eprintln!("{msg}"),
    }
}
