#[derive(Debug)]
pub enum Token {
    Number(f32),
    Operator(char),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, &str> {
    let mut result: Result<Vec<Token>, &str> = Result::Err("");
    let mut tokens: Vec<Token> = Vec::new();

    let chars: Vec<char> = input.chars().collect();
    let mut index = 0;
    let mut error = false;

    while index < chars.len() {
        let char = chars.get(index).unwrap();
        if (*char).is_ascii_whitespace() {
            index += 1;
            continue;
        }

        if char.is_digit(10) {
            let mut number = String::new();

            let mut next_char = chars.get(index).unwrap();
            while next_char.is_digit(10) || *next_char == '.' {
                number.push(*next_char);

                index += 1;
                next_char = match chars.get(index) {
                    Some(c) => c,
                    None => &'\0',
                };
            }

            let n = number.parse::<f32>().unwrap();
            tokens.push(Token::Number(n));
        } else if *char == '+' || *char == '-' || *char == '*' || *char == '/' || *char == '^' {
            tokens.push(Token::Operator(*char));
        } else {
            result = Result::Err("Unknown character encountered");
            error = true;
            break;
        }

        index += 1;
    }

    if !error {
        result = Result::Ok(tokens);
    }

    result
}

// pub fn dump(tokens: &[Token]) {
//     for token in tokens {
//         println!("{:?}", token);
//     }
// }
