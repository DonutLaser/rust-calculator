#[derive(Debug, Clone)]
pub enum Token {
    Number(f32),
    Operator(char),
    Identifier(String),
    LParen,
    RParen,
}

pub struct TokenList {
    data: Vec<Token>,
    cursor: usize,
}

impl TokenList {
    pub fn new(data: Vec<Token>) -> Self {
        TokenList { data, cursor: 0 }
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.cursor == self.data.len() {
            None
        } else {
            let token = self.data.get(self.cursor).unwrap();
            self.cursor += 1;
            Some(token.clone())
        }
    }

    pub fn peek(&self, step: Option<usize>) -> Option<Token> {
        let stepby = step.unwrap_or(0);

        if self.cursor + stepby >= self.data.len() {
            None
        } else {
            let token = self.data.get(self.cursor + stepby).unwrap();
            Some(token.clone())
        }
    }
}

pub fn tokenize(input: &str) -> Option<TokenList> {
    let mut result: Option<TokenList> = Option::None;
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

        if char.is_digit(10) || *char == '.' {
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

            match number.parse::<f32>() {
                Ok(n) => tokens.push(Token::Number(n)),
                Err(err) => {
                    eprintln!("Error: {}", err);
                    error = true;
                    break;
                }
            }

            index -= 1;
        } else if *char == '+' || *char == '-' || *char == '*' || *char == '/' || *char == '^' {
            tokens.push(Token::Operator(*char));
        } else if *char == '(' {
            tokens.push(Token::LParen);
        } else if *char == ')' {
            tokens.push(Token::RParen);
        } else if char.is_ascii_alphabetic() {
            let mut ident = String::new();

            let mut next_char = chars.get(index).unwrap();
            while next_char.is_ascii_alphabetic() {
                ident.push(*next_char);

                index += 1;
                next_char = match chars.get(index) {
                    Some(c) => c,
                    None => &'\0',
                };
            }

            tokens.push(Token::Identifier(ident));

            index -= 1;
        } else {
            eprintln!("Error: unknown character encountered, {}", char);
            error = true;
            break;
        }

        index += 1;
    }

    if !error {
        result = Option::Some(TokenList::new(tokens));
    }

    result
}
