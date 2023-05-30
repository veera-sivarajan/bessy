use std::io::Write;

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{prompt} ");
    let _ = std::io::stdout().flush();
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    let _ = input.pop();

    input
}

fn repl() {
    loop {
        let input = get_input("bessy>>");
        match core::evaluate(&input) {
            Ok(()) => continue,
            Err(msg) => eprintln!("{msg}"),
        }
    }
}

fn evaluate_file() {
    let input = include_str!("../../test/hello.lox");
    match core::evaluate(input) {
        Ok(()) => {},
        Err(msg) => eprintln!("{msg}"),
    }
}

fn main() {
    repl()
}
