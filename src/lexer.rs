use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq, Debug)]
pub enum Token {
    Number(f64),
    String(String),
    LParen,
    RParen,
}

#[derive(Debug)]
pub enum LexerError {
    UnknownSymbol(char),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut chars = input.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(c) = chars.peek() {
        if c.is_ascii_whitespace() {
            chars.next();
            continue;
        }

        let token = match c {
            '(' => {
                chars.next();
                Token::LParen
            }
            ')' => {
                chars.next();
                Token::RParen
            }
            ch if is_valid_symbol_char(ch) => tokenize_symbol(&mut chars)?,
            ch => return Err(LexerError::UnknownSymbol(ch.clone())),
        };

        tokens.push(token);
    }

    Ok(tokens)
}

fn tokenize_symbol(chars: &mut Peekable<Chars<'_>>) -> Result<Token, LexerError> {
    // parse until space
    let mut symbol = String::new();
    while let Some(c) = chars.next_if(|ch| is_valid_symbol_char(ch)) {
        symbol.push(c);
    }

    // Turn into number is possible, else it's a string
    match symbol.parse::<f64>() {
        Ok(num) => Ok(Token::Number(num)),
        Err(_) => Ok(Token::String(symbol)),
    }
}

fn is_valid_symbol_char(c: &char) -> bool {
    match c {
        '-' | '$' | '!' | '@' | '#' | '%' | '^' | '&' | '*' | '<' | '>' | '.' | '+' | '=' => true,
        ch if ch.is_ascii_alphanumeric() => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_symbol_string() {
        let mut input = "hello".chars().peekable();
        let expected = Token::String("hello".to_string());

        let result = tokenize_symbol(&mut input).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tokenize_symbol_number_1() {
        let mut input = "1234".chars().peekable();
        let expected = Token::Number(1234.0);

        let result = tokenize_symbol(&mut input).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tokenize_symbol_number_2() {
        let mut input = "1234.5678".chars().peekable();
        let expected = Token::Number(1234.5678);

        let result = tokenize_symbol(&mut input).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tokenize_symbol_number_negative() {
        let mut input = "-1234".chars().peekable();
        let expected = Token::Number(-1234.0);

        let result = tokenize_symbol(&mut input).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tokenize_1() {
        let input = "(hello)";
        let expected = vec![
            Token::LParen,
            Token::String("hello".to_string()),
            Token::RParen,
        ];

        let result = tokenize(&input).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tokenize_2() {
        let input = "(hello goodbye)";
        let expected = vec![
            Token::LParen,
            Token::String("hello".to_string()),
            Token::String("goodbye".to_string()),
            Token::RParen,
        ];

        let result = tokenize(&input).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tokenize_3() {
        let input = "(123 456)";
        let expected = vec![
            Token::LParen,
            Token::Number(123.0),
            Token::Number(456.0),
            Token::RParen,
        ];

        let result = tokenize(&input).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tokenize_4() {
        let input = "(+ (- 7 5) (* 10 4))";
        let expected = vec![
            Token::LParen,
            Token::String("+".to_string()),
            Token::LParen,
            Token::String("-".to_string()),
            Token::Number(7.0),
            Token::Number(5.0),
            Token::RParen,
            Token::LParen,
            Token::String("*".to_string()),
            Token::Number(10.0),
            Token::Number(4.0),
            Token::RParen,
            Token::RParen,
        ];

        let result = tokenize(&input).unwrap();
        assert_eq!(expected, result);
    }
}
