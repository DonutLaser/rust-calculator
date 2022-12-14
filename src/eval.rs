use crate::parser::{BinaryOperation, Constant, Expression, Function, Operator};
use std::f64::consts::PI;

pub fn eval(expr: Expression) -> Option<f32> {
    match expr {
        Expression::NumberLiteral(n) => Some(n),
        Expression::Constant(c) => eval_constant(c),
        Expression::FunctionCall(func) => eval_func(func),
        Expression::BinaryOp(op) => eval_binary_operation(op),
    }
}

fn eval_func(func: Function) -> Option<f32> {
    let value = eval(*func.argument);
    value?;

    let v = value.unwrap();

    match func.name.as_str() {
        "abs" => Some(v.abs()),
        "sin" => Some(v.sin()),
        "cos" => Some(v.cos()),
        "sqrt" => Some(v.sqrt()),
        _ => {
            println!("Error: unknown function");
            None
        }
    }
}

fn eval_constant(constant: Constant) -> Option<f32> {
    match constant {
        Constant::PI => Some(PI as f32),
    }
}

fn eval_binary_operation(op: BinaryOperation) -> Option<f32> {
    let left = eval(*op.left);
    let right = eval(*op.right);

    if left.is_none() || right.is_none() {
        return None;
    }

    let lhs = left.unwrap();
    let rhs = right.unwrap();

    match op.operator {
        Operator::Add => Some(lhs + rhs),
        Operator::Subtract => Some(lhs - rhs),
        Operator::Multiply => Some(lhs * rhs),
        Operator::Divide => Some(lhs / rhs),
        Operator::Power => Some(lhs.powf(rhs)),
    }
}
