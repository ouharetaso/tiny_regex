#[allow(dead_code)]

use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
pub enum Token {
    Char(char),
    LParen,
    RParen,
    LBracket,
    RBracket,
    Asterisk,
    Hyphen,
    VBar,
    Hat,
    Dot,
    EOF
}


impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Char(c) => write!(f, "{}", c),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::Asterisk => write!(f, "*"),
            Token::Hyphen => write!(f, "-"),
            Token::VBar => write!(f, "|"),
            Token::Hat => write!(f, "^"),
            Token::Dot => write!(f, "."),
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
            '[' => ret.push_back(Token::LBracket),
            ']' => ret.push_back(Token::RBracket),
            '-' => ret.push_back(Token::Hyphen),
            '^' => ret.push_back(Token::Hat),
            '.' => ret.push_back(Token::Dot),
            '\\' => ret.push_back(Token::Char(match char_indices.next().ok_or("backslash is not followed by any character")?.1 {
                'n' => '\n', // newline
                'r' => '\r', // carriage return
                't' => '\t', // tab character
                '0' => '\0', // null character
                _ => c
            })),
            _ => ret.push_back(Token::Char(c))
        }
    }

    ret.push_back(Token::EOF);

    Ok(ret)
}

