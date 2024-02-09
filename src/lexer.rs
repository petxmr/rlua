use regex::Regex;

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Number(f64),
    Operator(String),
    Let,
    Assignment,
    Semicolon,
}

pub fn lexer(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let identifier = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
    let number = Regex::new(r"^\d+(\.\d+)?").unwrap();
    let declaration = Regex::new(r"^let\b").unwrap();
    let operator = Regex::new(r"^[+\-*/]").unwrap();
    let assignment = Regex::new(r"^=").unwrap();
    let semicolon = Regex::new(r"^;").unwrap();

    let mut chars = source.trim();

    while !chars.is_empty() {
        if let Some(captures) = declaration.captures(chars) {
            tokens.push(Token::Let);
            chars = &chars[captures[0].len()..];
        } else if let Some(captures) = identifier.captures(chars) {
            let id = captures[0].to_string();
            tokens.push(Token::Identifier(id.clone()));
            chars = &chars[id.len()..];
        } else if let Some(captures) = number.captures(chars) {
            let num = captures[0].parse::<f64>().unwrap();
            tokens.push(Token::Number(num));
            chars = &chars[captures[0].len()..];
        } else if let Some(captures) = operator.captures(chars) {
            let op = captures[0].to_string();
            tokens.push(Token::Operator(op.clone()));
            chars = &chars[op.len()..];
        } else if let Some(captures) = assignment.captures(chars) {
            tokens.push(Token::Assignment);
            chars = &chars[captures[0].len()..];
        } else if let Some(captures) = semicolon.captures(chars) {
            tokens.push(Token::Semicolon);
            chars = &chars[captures[0].len()..];
        } else {
            panic!("Unexpected character: {}", chars.chars().next().unwrap());
        }

        chars = chars.trim();
    }

    tokens
}
