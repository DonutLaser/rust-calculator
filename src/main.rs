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

        let lex_result = lexer::tokenize(&input);
        if lex_result.is_none() {
            continue;
        }

        let parse_result = parser::parse(lex_result.unwrap());
        if parse_result.is_none() {
            continue;
        }

        let result = eval::eval(parse_result.unwrap());
        println!("{}", result);
    }
}
