use crate::lexer::Token;

use super::parser::{Declaration, Expression};

pub fn translate(declarations: Vec<Declaration>) -> String {
    let mut lua_code = String::new();

    for dec in declarations {
        match dec.initializer {
            Some(expr) => {
                lua_code.push_str(&format!(
                    "local {} = {};\n",
                    dec.variable,
                    translate_expression(expr)
                ));
            }
            None => {
                lua_code.push_str(&format!("local {};\n", dec.variable));
            }
        }
    }

    lua_code
}

fn translate_expression(expression: Expression) -> String {
    match expression {
        Expression::Number(num) => num.to_string(),
        Expression::Variable(var) => var,
        Expression::BinaryOperation(left, op, right) => {
            let left_str = translate_expression(*left);
            let right_str = translate_expression(*right);
            match op {
                Token::Operator(op) => match op.as_str() {
                    "+" => format!("{} + {}", left_str, right_str),
                    "-" => format!("{} - {}", left_str, right_str),
                    "/" => format!("{} / {}", left_str, right_str),
                    "*" => format!("{} * {}", left_str, right_str),
                    _ => panic!("Unsupported operator: {}", op),
                }
                _ => panic!("Expected operator"),
            }
        }
    }
}
