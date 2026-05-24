use std::{iter::Peekable, slice::Iter};
use test_strategy::Arbitrary;

use crate::lexer::Token;

#[derive(PartialEq, Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEndOfInput,
    UnexpectedEndOfList,
    ExpectedEOF,
}

#[derive(PartialEq, Debug, Arbitrary, Clone)]
pub enum Expr {
    Nil,
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<Expr>),
    Cons { car: Box<Expr>, cdr: Box<Expr> },
}

impl Expr {
    pub fn print(&self) -> String {
        let mut output = String::new();

        match self {
            Expr::Symbol(sym) => output.push_str(&format!("Symbol[{}]", sym)),
            Expr::Number(num) => output.push_str(&format!("Number[{}]", num)),
            Expr::String(s) => output.push_str(&format!("String[{}]", s)),
            Expr::Nil => output.push_str("nil"),
            Expr::Cons { car, cdr } => {
                output.push_str("Cons[");
                output.push_str("(");
                output.push_str(&car.print());
                output.push_str(" . ");
                output.push_str(&cdr.print());
                output.push_str(")]");
            }
            Expr::List(list) => {
                output.push_str("List[(");
                let s = list
                    .iter()
                    .map(|e| e.print())
                    .collect::<Vec<String>>()
                    .join(" ");
                output.push_str(&s);
                output.push_str("])");
            }
        }

        output
    }
}

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
        // Left Paren
        Some(Token::LParen) => parse_parens(tokens),
        Some(Token::Number(num)) => {
            tokens.next();
            Ok(Expr::Number(*num))
        }
        // String
        Some(Token::String(s)) => {
            tokens.next();
            Ok(Expr::String(s.clone()))
        }
        // Symbol
        Some(Token::Symbol(s)) => {
            tokens.next();
            Ok(Expr::Symbol(s.clone()))
        }
        // Unexpected token
        Some(token) => Err(ParseError::UnexpectedToken((*token).clone())),
        None => Err(ParseError::UnexpectedEndOfInput),
    }
}

fn parse_parens(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, ParseError> {
    expect(tokens, Token::LParen)?;

    // parse first element
    let first = parse_expr(tokens)?;

    if let Some(Token::Dot) = tokens.peek() {
        // if next token is DOT, then parse next then expect RPAREN. Final Expr is Cons
        let second = parse_expr(tokens)?;
        expect(tokens, Token::RParen)?;

        Ok(Expr::Cons {
            car: Box::new(first),
            cdr: Box::new(second),
        })
    } else {
        // Else it's a list
        let mut list: Vec<Expr> = Vec::new();
        list.push(first);

        while let Some(token) = tokens.peek()
            && Token::RParen != **token
        {
            list.push(parse_expr(tokens)?);
        }
        expect(tokens, Token::RParen)?;

        Ok(Expr::List(list))
    }
}

// consumes the token if it matches
fn expect(tokens: &mut Peekable<Iter<Token>>, token: Token) -> Result<(), ParseError> {
    match tokens.next_if(|t| matches!(t, token)) {
        Some(_) => Ok(()),
        None => Err(ParseError::UnexpectedToken(token)),
    }
}
