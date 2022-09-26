use crate::lexer::Token;

#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug)]
pub enum ParsedToken {
    NumberLiteral(f32),
    Operator(Operator, u16),
}

pub fn parse(lexer_tokens: Vec<Token>) -> Option<Vec<ParsedToken>> {
    let mut result: Option<Vec<ParsedToken>> = None;
    let mut tokens: Vec<ParsedToken> = Vec::new();

    let mut index = 0;
    let mut error = false;

    while index < lexer_tokens.len() {
        let token = lexer_tokens.get(index).unwrap();

        match token {
            Token::Number(n) => {
                tokens.push(ParsedToken::NumberLiteral(*n));
                index += 1;
            }
            Token::Operator('-') => {
                if index == 0 {
                    if lexer_tokens.len() > 1 {
                        let next_token = lexer_tokens.get(index + 1).unwrap();
                        if let Token::Number(n) = next_token {
                            tokens.push(ParsedToken::NumberLiteral(-(*n)));
                            index += 2;
                        } else {
                            eprintln!("Error: unexpected operator");
                            error = true;
                            break;
                        }
                    } else {
                        eprintln!("Error: expected number");
                        error = true;
                        break;
                    }
                } else {
                    let prev_token = lexer_tokens.get(index - 1).unwrap();
                    if let Token::Operator(_) = prev_token {
                        let next_token = lexer_tokens.get(index + 1).unwrap();
                        if let Token::Number(n) = next_token {
                            tokens.push(ParsedToken::NumberLiteral(-(*n)));
                            index += 2;
                        } else {
                            eprintln!("Error: unexpected operator");
                            error = true;
                            break;
                        }
                    } else {
                        tokens.push(ParsedToken::Operator(Operator::Subtract, 100));
                        index += 1;
                    }
                }
            }
            Token::Operator('+') => {
                tokens.push(ParsedToken::Operator(Operator::Add, 100));
                index += 1;
            }
            Token::Operator('*') => {
                tokens.push(ParsedToken::Operator(Operator::Multiply, 200));
                index += 1;
            }
            Token::Operator('/') => {
                tokens.push(ParsedToken::Operator(Operator::Divide, 200));
                index += 1;
            }
            Token::Operator('^') => {
                tokens.push(ParsedToken::Operator(Operator::Power, 250));
                index += 1;
            }
            Token::End => {
                index += 1;
            }
            _ => (),
        }
    }

    if !error {
        result = Some(tokens);
    }

    result
}

// pub fn dump(tokens: &[ParsedToken]) {
//     for token in tokens {
//         println!("{:?}", token);
//     }
// }
