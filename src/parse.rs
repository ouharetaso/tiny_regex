#[allow(dead_code)]

use std::collections::VecDeque;
use crate::token::*;

#[derive(Debug)]
pub enum Node {
    Char(char),
    Concat((Box<Node>, Box<Node>)),
    Union((Box<Node>, Box<Node>)),
    Repeat(Box<Node>)
}


pub fn parse(tokens: &mut VecDeque<Token>) -> Result<Node, String> {
    match expr(tokens) {
        Ok(root) => Ok(*root),
        Err(e) => Err(format!("Parse Error: {}", e))
    }
}


fn character(c: char) -> Box<Node> {
    Box::new(Node::Char(c))
}

fn concat(node1: Node, node2: Node) -> Box<Node> {
    Box::new(Node::Concat((Box::new(node1), Box::new(node2))))
}

fn union(node1: Node, node2: Node) -> Box<Node> {
    Box::new(Node::Union((Box::new(node1), Box::new(node2))))
}

fn repeat(node: Node) -> Box<Node> {
    Box::new(Node::Repeat(Box::new(node)))
}

fn consume(tokens: &mut VecDeque<Token>, token: Token) -> Result<(), String> {
    let next = tokens.pop_front().ok_or("Unexpected end of tokens".to_string())?;

    if next == token {
        Ok(())
    }
    else {
        Err(format!("Expected token: {}, found: {}", token, next))
    }
}

/*
expr    := subexpr EOF
subexpr := seq '|' subexpr | seq
seq     := subseq | ''
subseq  := star subseq | star
star    := factor '*' | factor
factor  := '(' subexpr ')' | CHARACTER
*/


fn factor(tokens: &mut VecDeque<Token>) -> Result<Box<Node>, String> {
    let token = tokens.pop_front().ok_or( "Unexpected end of tokens".to_string())?;

    // factor := '(' subexpr ')'
    if token == Token::LParen {
        let ret = Ok(subexpr(tokens)?);
        consume(tokens, Token::RParen)?;
        ret
    }
    // factor := CHARACTER
    else if let Token::Char(c) = token {
        Ok(character(c))
    }
    // error
    else {
        Err(format!("unexpected token: {}", token))
    }
}

fn star(tokens: &mut VecDeque<Token>) -> Result<Box<Node>, String> {
    let node = factor(tokens)?;
    
    if let Some(token) = tokens.front() {
        // star := factor '*'
        if *token == Token::Asterisk {
            consume(tokens, Token::Asterisk)?;
            let ret = Ok(repeat(*node));
            ret
        }
        // star := factor
        else {
            Ok(node)
        }
    }
    else {
        Ok(node)
    }
}

fn seq(tokens: &mut VecDeque<Token>) -> Result<Box<Node>, String> {
    // seq := subseq | ''
    if let Some(token) = tokens.front() {
        match *token {
            // seq := subseq
            Token::LParen | Token::Char(_) => {
                subseq(tokens)
            }
            // seq := ''
            _ => {
                Ok(character('\0'))
            }
        }
    }
    else {
        Err("Unexpected end of tokens".to_string())
    }
}



fn subseq(tokens: &mut VecDeque<Token>) -> Result<Box<Node>, String> {
    // subseq  := star subseq | star
    let node = star(tokens)?;

    if let Some(token) = tokens.front() {
        match *token {
            // subseq := star subseq
            Token::LParen | Token::Char(_) => {
                Ok(concat(*node, *subseq(tokens)?))
            }
            // subseq := star
            _ => {
                Ok(node)
            }
        }
    }
    else {
        Err("Unexpected end of tokens".to_string())
    }
}

fn subexpr(tokens: &mut VecDeque<Token>) -> Result<Box<Node>, String> {
    // subexpr := seq '|' subexpr | seq
    let node = seq(tokens)?;

    if let Some(token) = tokens.front() {
        match *token {
            // subexpr := seq '|' subexpr
            Token::VBar => {
                consume(tokens, Token::VBar)?;
                Ok(union(*node, *subexpr(tokens)?))
            }
            // subexpr := seq
            _ => {
                Ok(node)
            }
        }
    }
    else {
        Err("Unexpected end of tokens".to_string())
    }
}


fn expr(tokens: &mut VecDeque<Token>) -> Result<Box<Node>, String> {
    // expr := subexpr EOF
    let node = subexpr(tokens)?;

    consume(tokens, Token::EOF)?;

    if tokens.is_empty() {
        Ok(node)
    }
    else {
        Err("Unexpected end of tokens".to_string())
    }
}