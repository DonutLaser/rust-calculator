use crate::lexer::Token;
use std::collections::HashMap;

pub fn eval(tokens: Vec<Token>) -> u32 {
    let mut precedence: HashMap<char, u8> = HashMap::new();
    precedence.insert('+', 100);
    precedence.insert('-', 100);
    precedence.insert('*', 200);
    precedence.insert('/', 200);
    precedence.insert('^', 230);

    let mut operators: Vec<char> = Vec::new();
    let mut operands: Vec<u32> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(n) => operands.push(n),
            Token::Operator(c) => {
                if operators.is_empty()
                    || precedence.get(operators.last().unwrap()) < precedence.get(&c)
                {
                    operators.push(c);
                } else {
                    let right = operands.pop().unwrap();
                    let left = operands.pop().unwrap();

                    let operator = operators.pop().unwrap();

                    match operator {
                        '+' => operands.push(left + right),
                        '-' => operands.push(left - right),
                        '*' => operands.push(left * right),
                        '/' => operands.push(left / right),
                        '^' => operands.push(left.pow(right)),
                        _ => (),
                    };

                    operators.push(c);
                }
            }
        }
    }

    while !operators.is_empty() {
        let right = operands.pop().unwrap();
        let left = operands.pop().unwrap();

        let operator = operators.pop().unwrap();

        match operator {
            '+' => operands.push(left + right),
            '-' => operands.push(left - right),
            '*' => operands.push(left * right),
            '/' => operands.push(left / right),
            '^' => operands.push(left.pow(right)),
            _ => (),
        };
    }

    *operands.get(0).unwrap()
}
