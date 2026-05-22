use std::{iter::Peekable, slice::Iter};

use crate::lexer::Token;

#[derive(PartialEq, Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEndOfInput,
    UnexpectedEndOfList,
    ExpectedEOF,
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Nil,
    Number(f64),
    Symbol(String),
    String(String),
    List(Cons),
    Cons(Cons),
}

#[derive(PartialEq, Debug)]
pub struct Cons {
    car: Box<Expr>,
    cdr: Box<Expr>,
}

impl Expr {
    pub fn print(&self) -> String {
        let mut output = String::new();

        match self {
            Expr::Number(num) => output.push_str(&num.to_string()),
            Expr::Symbol(sym) => output.push_str(sym),
            Expr::String(s) => output.push_str(&format!("\"{}\"", s).to_string()),
            Expr::Cons(cons) => {
                output.push('(');
                output.push_str(&cons.car.print());
                output.push('.');
                output.push_str(&cons.cdr.print());
                output.push(')');
            }
            Expr::Nil => output.push_str("nil"),
        }

        output
    }
}    Visual: Think of it as a single domino or a box split into two halves: [ Left | Right ].
Contents: The left side (CAR) holds a value. The right side (CDR) holds another value or a pointer to the next cell.
Notation: Written in dotted notation as (A . B).



pub fn parse(tokens: &[Token]) -> Result<Expr, ParseError> {
    let mut tokens_peekable = tokens.iter().peekable();
    let expr = parse_expr(&mut tokens_peekable);

    // Make sure only EOF is left
    match tokens_peekable.peek() {
        Some(Token::EOF) => (),
        Some(other) => return Err(ParseError::UnexpectedToken((*other).clone())),
        None => return Err(ParseError::ExpectedEOF),
    }

    expr
}

fn parse_expr(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, ParseError> {
    match tokens.peek() {
        Some(Token::LParen) => parse_list(tokens),
        Some(Token::Number(num)) => {
            tokens.next();
            Ok(Expr::Number(*num))
        }
        Some(Token::String(s)) => {
            tokens.next();
            Ok(Expr::String(s.clone()))
        }
        Some(Token::Symbol(s)) => {
            tokens.next();
            Ok(Expr::Symbol(s.clone()))
        }
        Some(token) => Err(ParseError::UnexpectedToken((*token).clone())),
        None => Err(ParseError::UnexpectedEndOfInput),
    }
}

fn parse_list(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, ParseError> {
    expect(tokens, Token::LParen)?;

    // Parse all elements of list, then build the cons cells bottom up
    let mut stack: Vec<Expr> = Vec::new();
    while !matches!(tokens.peek(), Some(Token::RParen)) {
        stack.push(parse_expr(tokens)?);
    }

    let mut lower_cons = Cons {
        car: Box::new(stack.pop().ok_or(ParseError::UnexpectedEndOfList)?),
        cdr: Box::new(Expr::Nil),
    };
    while let Some(expr) = stack.pop() {
        let root_cons = Cons {
            car: Box::new(expr),
            cdr: Box::new(Expr::Cons(lower_cons)),
        };

        lower_cons = root_cons;
    }

    expect(tokens, Token::RParen)?;

    Ok(Expr::List(lower_cons))
}

// consumes the token if it matches
fn expect(tokens: &mut Peekable<Iter<Token>>, token: Token) -> Result<(), ParseError> {
    match tokens.next_if(|t| matches!(t, token)) {
        Some(_) => Ok(()),
        None => Err(ParseError::UnexpectedToken(token)),
    }
}
