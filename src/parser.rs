use std::{iter::Peekable, slice::Iter};

use crate::lexer::Token;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Nil,
    Number(f64),
    Symbol(String),
    List(Vec<Expr>),
}

impl Expr {
    pub fn print(&self) -> String {
        let mut output = String::new();

        match self {
            Expr::Number(num) => output.push_str(&num.to_string()),
            Expr::Symbol(s) => output.push_str(s),
            Expr::List(list) => {
                let s = list
                    .iter()
                    .map(|l| l.print())
                    .collect::<Vec<String>>()
                    .join(" ");

                output.push('(');
                output.push_str(&s);
                output.push(')');
            }
            Expr::Nil => output.push_str("nil"),
        }

        output
    }
}

#[derive(PartialEq, Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEndOfInput,
}

pub fn parse(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, ParseError> {
    match tokens.peek() {
        Some(Token::LParen) => parse_list(tokens),
        Some(Token::Number(num)) => {
            tokens.next();
            Ok(Expr::Number(*num))
        }
        Some(Token::String(s)) => {
            tokens.next();
            Ok(Expr::Symbol(s.clone()))
        }
        Some(token) => Err(ParseError::UnexpectedToken((*token).clone())),
        None => Err(ParseError::UnexpectedEndOfInput),
    }
}

fn parse_list(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, ParseError> {
    expect(tokens, Token::LParen)?;

    let mut list: Vec<Expr> = Vec::new();

    while !matches!(tokens.peek(), Some(Token::RParen)) {
        let expr = parse(tokens)?;
        list.push(expr);
    }

    expect(tokens, Token::RParen)?;

    Ok(Expr::List(list))
}

// consumes the token if it matches
fn expect(tokens: &mut Peekable<Iter<Token>>, token: Token) -> Result<(), ParseError> {
    match tokens.next_if(|t| matches!(t, token)) {
        Some(_) => Ok(()),
        None => Err(ParseError::UnexpectedToken(token)),
    }
}
