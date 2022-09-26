use crate::parser::{Operator, ParsedToken};

pub type Operation = (Operator, u16);

pub fn eval(tokens: Vec<ParsedToken>) -> f32 {
    let mut operators: Vec<Operation> = Vec::new();
    let mut operands: Vec<f32> = Vec::new();

    for token in tokens {
        match token {
            ParsedToken::NumberLiteral(n) => operands.push(n),
            ParsedToken::Operator(op, precedence) => {
                if !operators.is_empty() && operators.last().unwrap().1 >= precedence {
                    let right = operands.pop().unwrap();
                    let left = operands.pop().unwrap();

                    let operator = operators.pop().unwrap();

                    match operator.0 {
                        Operator::Add => operands.push(left + right),
                        Operator::Subtract => operands.push(left - right),
                        Operator::Multiply => operands.push(left * right),
                        Operator::Divide => operands.push(left / right),
                        Operator::Power => operands.push(left.powf(right)),
                    };
                }

                operators.push((op, precedence));
            }
        }
    }

    while !operators.is_empty() {
        let right = operands.pop().unwrap();
        let left = operands.pop().unwrap();

        let operator = operators.pop().unwrap();

        match operator.0 {
            Operator::Add => operands.push(left + right),
            Operator::Subtract => operands.push(left - right),
            Operator::Multiply => operands.push(left * right),
            Operator::Divide => operands.push(left / right),
            Operator::Power => operands.push(left.powf(right)),
        };
    }

    *operands.get(0).unwrap()
}
