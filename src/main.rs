use std::io::{stdin, stdout, Write};
mod eval;
mod lexer;
mod parser;

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

        if input.is_empty() {
            continue;
        }

        let tokens_list = lexer::tokenize(&input);
        if tokens_list.is_none() {
            continue;
        }

        let expression = parser::parse(&mut tokens_list.unwrap());
        if expression.is_none() {
            continue;
        }

        let result = eval::eval(expression.unwrap());
        if result.is_none() {
            continue;
        }

        println!("{}", result.unwrap());
    }
}
