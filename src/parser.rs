use crate::lexer::{Token, TokenList};

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug)]
pub struct BinaryOperation {
    pub operator: Operator,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub argument: Box<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    NumberLiteral(f32),
    BinaryOp(BinaryOperation),
    FunctionCall(Function),
}

pub fn parse(tokens: &mut TokenList) -> Option<Expression> {
    let mut operators: Vec<Operator> = Vec::new();
    let mut operands: Vec<Expression> = Vec::new();

    let operand = parse_operand(tokens);
    operands.push(operand?);

    let mut next_token = tokens.peek(None);
    while next_token.is_some() {
        if let Token::RParen = next_token.unwrap() {
            break;
        }

        let operator = parse_operator(tokens);
        let precedence = get_operator_precedence(operator?);

        while !operators.is_empty()
            && get_operator_precedence(*operators.last().unwrap()) >= precedence
        {
            let op = operators.pop().unwrap();
            let right = operands.pop().unwrap();
            let left = operands.pop().unwrap();

            let binary_op = BinaryOperation {
                operator: op,
                left: Box::new(left),
                right: Box::new(right),
            };

            operands.push(Expression::BinaryOp(binary_op));
        }

        operators.push(operator.unwrap());

        let next_operand = parse_operand(tokens);
        operands.push(next_operand?);

        next_token = tokens.peek(None);
    }

    while !operators.is_empty() {
        let op = operators.pop().unwrap();
        let right = operands.pop().unwrap();
        let left = operands.pop().unwrap();

        let binary_op = BinaryOperation {
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
        };

        operands.push(Expression::BinaryOp(binary_op));
    }

    Some(operands.pop().unwrap())
}

fn parse_operand(tokens: &mut TokenList) -> Option<Expression> {
    let next_token = tokens.peek(None);
    if next_token.is_none() {
        println!("Error: unexpected end of expression");
        return None;
    }

    match next_token.unwrap() {
        Token::Number(n) => {
            _ = tokens.next();
            Some(Expression::NumberLiteral(n))
        }
        Token::Identifier(_) => parse_identifier(tokens),
        Token::Operator('-') => {
            _ = tokens.next();
            let res = parse_operand(tokens)?;
            if let Expression::NumberLiteral(n) = res {
                Some(Expression::NumberLiteral(-n))
            } else {
                None
            }
        }
        _ => {
            println!("Error: unexpected token {:?}", tokens.next().unwrap());
            None
        }
    }
}

fn parse_operator(tokens: &mut TokenList) -> Option<Operator> {
    let next_token = tokens.next();
    if next_token.is_none() {
        println!("Error: unexpected end of expression");
        return None;
    }

    let t = next_token.unwrap();
    match t {
        Token::Operator(op) => match op {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Subtract),
            '*' => Some(Operator::Multiply),
            '/' => Some(Operator::Divide),
            '^' => Some(Operator::Power),
            _ => {
                println!("Error: unexpected operator {}", op);
                None
            }
        },
        _ => {
            println!("Error: unexpected token {:?}", t);
            None
        }
    }
}

fn parse_identifier(tokens: &mut TokenList) -> Option<Expression> {
    // For now there aren't any identifiers that are not functions
    parse_function(tokens).map(Expression::FunctionCall)
}

fn parse_function(tokens: &mut TokenList) -> Option<Function> {
    let mut next_token = tokens.next();
    if next_token.is_none() {
        println!("Error: unexpected end of expression");
        return None;
    }

    let name = if let Token::Identifier(ident) = next_token.unwrap() {
        ident
    } else {
        println!("Error: expected identifier");
        return None;
    };

    next_token = tokens.next();
    if next_token.is_none() {
        println!("Error: unexpected end of expression");
        return None;
    }

    if !matches!(next_token.unwrap(), Token::LParen) {
        println!("Error: expected '('");
        return None;
    }

    next_token = tokens.peek(None);
    if next_token.is_none() {
        println!("Error: unexpected end of expression");
        return None;
    }

    let expression = parse(tokens);
    let arg = expression?;

    next_token = tokens.next();
    if next_token.is_none() {
        println!("Error: unexpected end of expression");
        return None;
    }

    if let Token::RParen = next_token.unwrap() {
        Some(Function {
            name,
            argument: Box::new(arg),
        })
    } else {
        println!("Error: expected ')'");
        None
    }
}

fn get_operator_precedence(op: Operator) -> u16 {
    match op {
        Operator::Add | Operator::Subtract => 100,
        Operator::Multiply | Operator::Divide => 200,
        Operator::Power => 250,
    }
}
