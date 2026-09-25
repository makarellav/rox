use std::{default, iter::Peekable, str::Chars};

use crate::{error::RoxError, reader::Source};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

pub struct Tokens {
    pub data: Vec<Token>,
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

fn get_single_char_token(ch: char) -> Option<TokenType> {
    match ch {
        '(' => Some(TokenType::LParen),
        ')' => Some(TokenType::RParen),
        '{' => Some(TokenType::LCurly),
        '}' => Some(TokenType::RCurly),
        ',' => Some(TokenType::Comma),
        '.' => Some(TokenType::Dot),
        '-' => Some(TokenType::Minus),
        '+' => Some(TokenType::Plus),
        ';' => Some(TokenType::Semicolon),
        '*' => Some(TokenType::Star),
        _ => None,
    }
}

pub fn tokenize(source: &Source) -> Tokens {
    let mut it = source.raw.chars().peekable();

    let mut tokens = Tokens {
        data: vec![],
        errors: vec![],
    };

    let mut line = 1;

    while let Some(ch) = it.next() {
        if let Some(token_type) = get_single_char_token(ch) {
            tokens
                .data
                .push(Token::new(token_type, ch.to_string(), line));

            continue;
        }

        match ch {
            '!' | '=' | '>' | '<' => {
                let (single_type, double_type, double_lexeme) = match ch {
                    '!' => (TokenType::Bang, TokenType::NotEqual, "!="),
                    '=' => (TokenType::Equal, TokenType::EqualEqual, "=="),
                    '>' => (TokenType::Greater, TokenType::GreaterEqual, ">="),
                    '<' => (TokenType::Less, TokenType::LessEqual, "<="),
                    _ => unreachable!(),
                };

                if it.peek() == Some(&'=') {
                    it.next();

                    tokens
                        .data
                        .push(Token::new(double_type, double_lexeme.to_string(), line));
                } else {
                    tokens
                        .data
                        .push(Token::new(single_type, ch.to_string(), line));
                }
            }
            '/' => {
                if it.peek() == Some(&'/') {
                    it.next();

                    while let Some(&next_ch) = it.peek() {
                        if next_ch == '\n' {
                            break;
                        }

                        it.next();
                    }
                } else {
                    tokens
                        .data
                        .push(Token::new(TokenType::Slash, ch.to_string(), line));
                }
            }
            ' ' | '\t' | '\r' => {}
            '\n' => line += 1,
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
