use crate::lexer::Token;

#[derive(PartialEq, Debug)]
pub enum ParseError {
    NoMatches,
    UnexpectedToken(Token),
    UnexpectedEOF,
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    Atom(Atom),
    List { car: Box<Expr>, cdr: Box<Expr> },
}

#[derive(PartialEq, Debug)]
pub enum Atom {
    Nil,
    String(String),
    Number(f64),
}

pub fn parse(tokens: &[Token]) -> Result<(Expr, &[Token]), ParseError> {
    parse_expr(tokens)
}

fn parse_expr(tokens: &[Token]) -> Result<(Expr, &[Token]), ParseError> {
    // if LPAREN, parse list
    // else, parse atom
    match tokens.split_first() {
        Some((Token::LParen, rest)) => parse_list(rest),
        Some((atom, rest)) => parse_atom(tokens),
        None => Ok((Expr::Atom(Atom::Nil), &[])),
    }
}

fn parse_list(tokens: &[Token]) -> Result<(Expr, &[Token]), ParseError> {}

fn parse_atom(tokens: &[Token]) -> Result<(Expr, &[Token]), ParseError> {
    match tokens.split_first() {
        Some((Token::Number(num), rest)) => Ok((Expr::Atom(Atom::Number(*num)), rest)),
        Some((Token::String(s), rest)) => Ok((Expr::Atom(Atom::String(s.clone())), rest)),
        _ => Err(ParseError::NoMatches),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_atom() {
        let input = vec![Token::Number(123.456)];
        let expected = Expr::Atom(Atom::Number(123.456));

        let (result, rest) = parse_atom(&input).unwrap();
        assert_eq!(expected, result);
        assert_eq!(0, rest.iter().count());
    }

    #[test]
    fn test_parse_string() {
        let input = vec![Token::String("hello".to_string())];
        let expected = Expr::Atom(Atom::String("hello".to_string()));

        let (result, rest) = parse_atom(&input).unwrap();
        assert_eq!(expected, result);
        assert_eq!(0, rest.iter().count());
    }
}
