use std::iter::Peekable;
use std::str::Chars;
use test_strategy::Arbitrary;

#[derive(PartialEq, Debug, Clone, Arbitrary)]
pub enum Token {
    Number(f64),
    String(String),
    Symbol(String),
    LParen,
    RParen,
    Dot,
    EOF,
}

impl Token {
    pub fn print(&self) -> String {
        let output: &str = match self {
            Token::Dot => " . ",
            Token::Number(num) => &num.to_string(),
            Token::String(s) => s,
            Token::Symbol(sym) => sym,
            Token::EOF => "",
            Token::LParen => "(",
            Token::RParen => ")",
        };

        output.to_string()
    }
}

#[derive(Debug)]
pub enum LexerError {
    UnknownSymbol(char),
    InvalidNumber(String),
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
            '"' => tokenize_string(&mut chars)?,
            '.' => {
                chars.next();
                Token::Dot
            }
            ch if is_valid_symbol_char(ch) => tokenize_symbol_or_number(&mut chars)?,
            ch => return Err(LexerError::UnknownSymbol(ch.clone())),
        };

        tokens.push(token);
    }

    // Add EOF at end
    tokens.push(Token::EOF);

    Ok(tokens)
}

fn tokenize_string(chars: &mut Peekable<Chars<'_>>) -> Result<Token, LexerError> {
    expect(chars, '"')?;
    let mut string = String::new();
    while let Some(c) = chars.next_if(|ch| ch.is_ascii_alphabetic()) {
        string.push(c);
    }
    expect(chars, '"')?;

    Ok(Token::String(string))
}

fn tokenize_symbol_or_number(chars: &mut Peekable<Chars<'_>>) -> Result<Token, LexerError> {
    // parse until space
    let mut symbol = String::new();
    while let Some(c) = chars.next_if(|ch| is_valid_symbol_char(ch)) {
        symbol.push(c);
    }

    match symbol.parse::<f64>() {
        Ok(num) => Ok(Token::Number(num)),
        Err(_) => Ok(Token::Symbol(symbol)),
    }
}

fn is_valid_symbol_char(c: &char) -> bool {
    match c {
        '-' | '$' | '!' | '@' | '#' | '%' | '^' | '&' | '*' | '<' | '>' | '.' | '+' | '=' => true,
        ch if ch.is_ascii_alphanumeric() => true,
        _ => false,
    }
}

// consumes the token if it matches
fn expect(chars: &mut Peekable<Chars<'_>>, char: char) -> Result<(), LexerError> {
    match chars.next_if(|c| matches!(c, char)) {
        Some(_) => Ok(()),
        None => Err(LexerError::UnknownSymbol(char)),
    }
}
