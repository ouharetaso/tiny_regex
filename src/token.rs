#[allow(dead_code)]

use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
pub enum Token {
    Char(char),
    LParen,
    RParen,
    Asterisk,
    VBar,
    EOF
}


impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Char(c) => write!(f, "{}", c),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Asterisk => write!(f, "*"),
            Token::VBar => write!(f, "|"),
            Token::EOF => write!(f, "EOF")
        }
    }
}


pub fn tokenize(s: &String) -> Result<VecDeque<Token>, String> {
    let mut ret = VecDeque::<Token>::new();
    let mut char_indices = s.char_indices();

    while let Some((_i, c)) = char_indices.next() {
        match c {
            '(' => ret.push_back(Token::LParen),
            ')' => ret.push_back(Token::RParen),
            '*' => ret.push_back(Token::Asterisk),
            '|' => ret.push_back(Token::VBar),
            '\\' => ret.push_back(Token::Char(char_indices.next().unwrap_or((0, c)).1)),
            _ => ret.push_back(Token::Char(c))
        }
    }

    ret.push_back(Token::EOF);

    Ok(ret)
}

