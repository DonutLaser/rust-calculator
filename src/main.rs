use std::io::{stdin, stdout, Write};
mod eval;
mod lexer;

fn get_user_input() -> String {
    let mut buffer = String::new();

    print!("> ");
    let _ = stdout().flush();

    let mut result = match stdin().read_line(&mut buffer) {
        Ok(_n) => buffer,
        Err(_error) => String::from(""),
    };

    if let Some('\n') = result.chars().next_back() {
        result.pop();
    }
    if let Some('\r') = result.chars().next_back() {
        result.pop();
    }

    result
}

fn main() {
    loop {
        let input = get_user_input();
        if input.eq(&String::from("quit")) {
            break;
        }

        if let Some(tokens) = lexer::tokenize(&input) {
            let result = eval::eval(tokens);
            println!("{}", result);
        }
    }
}
