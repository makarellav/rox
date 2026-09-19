use std::default;

use crate::{error::RoxError, reader::Source};

pub enum TokenType {
    LParen,
    RParen,
    LCurly,
    RCurly,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Slash,
    Bang,
    Equal,
    Less,
    Greater,
    NotEqual,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    String,
    Number,
    Identifier,
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

pub struct Tokens {
    data: Vec<Token>,
    errors: Vec<RoxError>,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

pub fn tokenize(source: &Source) -> Tokens {
    let mut it = source.raw.chars().peekable();

    let mut tokens = Tokens {
        data: vec![],
        errors: vec![],
    };

    let mut _line = 1;

    while let Some(ch) = it.next() {
        match ch {
            '(' => tokens
                .data
                .push(Token::new(TokenType::LParen, ch.to_string(), 0)),
            ')' => tokens
                .data
                .push(Token::new(TokenType::RParen, ch.to_string(), 0)),
            '{' => tokens
                .data
                .push(Token::new(TokenType::LCurly, ch.to_string(), 0)),
            '}' => tokens
                .data
                .push(Token::new(TokenType::RCurly, ch.to_string(), 0)),
            ',' => tokens
                .data
                .push(Token::new(TokenType::Comma, ch.to_string(), 0)),
            '.' => tokens
                .data
                .push(Token::new(TokenType::Dot, ch.to_string(), 0)),

            '-' => tokens
                .data
                .push(Token::new(TokenType::Minus, ch.to_string(), 0)),
            '+' => tokens
                .data
                .push(Token::new(TokenType::Plus, ch.to_string(), 0)),
            ';' => tokens
                .data
                .push(Token::new(TokenType::Semicolon, ch.to_string(), 0)),
            '*' => tokens
                .data
                .push(Token::new(TokenType::Star, ch.to_string(), 0)),
            _ => {}
        }
    }

    tokens
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_tokenize() {
//         let result = tokenize();
//         assert_eq!(result, ());
//     }
// }
