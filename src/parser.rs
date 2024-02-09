use super::lexer::Token;

#[derive(Debug)]
pub struct Declaration {
    pub variable: String,
    pub initializer: Option<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Number(f64),
    Variable(String),
    BinaryOperation(Box<Expression>, Token, Box<Expression>),
}

pub fn parse(tokens: Vec<Token>) -> Vec<Declaration> {
    let mut declarations: Vec<Declaration> = Vec::new();
    let mut iter = tokens.into_iter().peekable();

    while let Some(token) = iter.next() {
        match token {
            Token::Let => {
                if let Some(Token::Identifier(variable)) = iter.next() {
                    if let Some(Token::Assignment) = iter.peek() {
                        iter.next();

                        let expression = parse_expression(&mut iter);
                        declarations.push(Declaration {
                            variable,
                            initializer: Some(expression),
                        });
                    } else {
                        declarations.push(Declaration {
                            variable,
                            initializer: None,
                        });
                    }
                } else {
                    panic!("Expected an identifier after 'let'");
                }
            }
            _ => {}
        }
    }

    declarations
}

fn parse_expression(iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Expression {
    let mut left = parse_operand(iter);

    while let Some(token) = iter.peek() {
        match token {
            Token::Operator(_) => {
                let op = iter.next().unwrap();
                let right = parse_operand(iter);
                left = Expression::BinaryOperation(Box::new(left), op, Box::new(right));
            }
            _ => break,
        }
    }

    left
}

fn parse_operand(iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Expression {
    match iter.next() {
        Some(Token::Number(num)) => Expression::Number(num),
        Some(Token::Identifier(variable)) => Expression::Variable(variable),
        _ => panic!("Expression expected"),
    }
}
