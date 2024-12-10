use std::collections::HashSet;
#[allow(dead_code)]

use std::collections::VecDeque;
use crate::token::*;

#[derive(Debug)]
pub enum Node {
    Char(char),
    Concat((Box<Node>, Box<Node>)),
    Union((Box<Node>, Box<Node>)),
    Repeat(Box<Node>),
    NegChar(HashSet<char>)
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

fn neg_character(c: char) -> HashSet<char> {
    let mut set = HashSet::new();
    set.insert(c);
    set
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

fn build_union_btree(start_char: char, end_char: char) -> Box<Node> {
    let diff = (end_char as u32).checked_sub(start_char as u32).unwrap();
    if diff == 0 {
        character(start_char)
    }
    else if diff == 1 {
        union(*character(start_char), *character(end_char))
    }
    else {
        let mid = (start_char as u32 + end_char as u32) / 2;
        let node1 = build_union_btree(start_char, char::from_u32(mid).unwrap());
        let node2 = build_union_btree(char::from_u32(mid + 1).unwrap(), end_char);
        union(*node1, *node2)
    }
}

/*
expr            := subexpr EOF
subexpr         := seq '|' subexpr | seq
seq             := subseq | ''
subseq          := star subseq | star
star            := factor '*' | factor
factor          := '(' subexpr ')' | CHARACTER | '[' charset_inner ']'
charset_inner   := CHARACTER charset_inner | CHARACTER '-' CHARACTER charset_inner | ''
*/


fn charset_inner(tokens: &mut VecDeque<Token>) -> Result<Box<Node>, String> {
    let token = tokens.pop_front().ok_or("Unexpected end of tokens".to_string())?;

    if let Token::Char(c) = token {
        // charset_inner := CHARACTER charset_inner
        if let Some(&Token::Char(_next)) = tokens.front() {
            Ok(union(*character(c), *charset_inner(tokens)?))
        }
        // charset_inner := CHARACTER '-' CHARACTER charset_inner
        else if let Some(&Token::Hyphen) = tokens.front() {
            consume(tokens, Token::Hyphen)?;
            let start_char = c;
            let end_char = if let Token::Char(cc) = tokens.pop_front().ok_or("Unexpected end of tokens".to_string())? {
                cc
            }
            else {
                return Err(format!("Unexpected meta character"));
            };

            let node1 = build_union_btree(start_char, end_char);

            if let Some(Token::RBracket) = tokens.front() {
                Ok(node1)
            }
            else {
                let node2 = charset_inner(tokens)?;
                Ok(union(*node1, *node2))
            }
        }
        // end of charset_inner
        else if let Some(&Token::RBracket) = tokens.front() {
            Ok(character(c))
        }
        // unexpected token
        else if let Some(t) = tokens.front() {
            Err(format!("Unexpected token \"{}\"", t))
        }
        // end of token
        else {
            Err("Unexpected end of tokens".to_string())
        }
    }
    else {
        Err(format!("Unexpected token \"{}\"", token))
    }
}


fn charset_inner_neg(tokens: &mut VecDeque<Token>) -> Result<HashSet<char>, String> {
    let token = tokens.pop_front().ok_or("Unexpected end of tokens".to_string())?;

    if let Token::Char(c) = token {
        // charset_inner := CHARACTER charset_inner
        if let Some(&Token::Char(_next)) = tokens.front() {
            let set1 = neg_character(c);
            let set2 = charset_inner_neg(tokens)?;

            Ok(set1.union(&set2).cloned().collect())
        }
        // charset_inner := CHARACTER '-' CHARACTER charset_inner
        else if let Some(&Token::Hyphen) = tokens.front() {
            consume(tokens, Token::Hyphen)?;
            let start_char = c;
            let end_char = if let Token::Char(cc) = tokens.pop_front().ok_or("Unexpected end of tokens".to_string())? {
                cc
            }
            else {
                return Err(format!("Unexpected meta character"));
            };

            let set1 = {
                let mut set = HashSet::new();
                for c in start_char..=end_char {
                    set.insert(c);
                }
                set
            };

            if let Some(Token::RBracket) = tokens.front() {
                Ok(set1)
            }
            else {
                let set2 = charset_inner_neg(tokens)?;
                Ok(set1.union(&set2).cloned().collect())
            }
        }
        // end of charset_inner
        else if let Some(&Token::RBracket) = tokens.front() {
            Ok(neg_character(c))
        }
        // unexpected token
        else if let Some(t) = tokens.front() {
            Err(format!("Unexpected token \"{}\"", t))
        }
        // end of token
        else {
            Err("Unexpected end of tokens".to_string())
        }
    }
    else {
        Err(format!("Unexpected token \"{}\"", token))
    }
}


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
    // factor := '[' charset_inner ']' | '[' '^' charset_inner ']'
    else if token == Token::LBracket {
        // factor := '[' '^' charset_inner ']'
        if let Some(&Token::Hat) = tokens.front() {
            consume(tokens, Token::Hat)?;
            let set = charset_inner_neg(tokens)?;
            consume(tokens, Token::RBracket)?;
            Ok(Box::new(Node::NegChar(set)))
        }
        // factor := '[' charset_inner ']'
        else {
            let node = charset_inner(tokens)?;
            consume(tokens, Token::RBracket)?;
            Ok(node)
        }
    }
    // factor := '.'
    else if token == Token::Dot {
        Ok(Box::new(Node::NegChar(HashSet::new())))
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
            Token::LParen | Token::Char(_) | Token::LBracket | Token::Dot => {
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
            Token::LParen | Token::Char(_) | Token::LBracket | Token::Dot => {
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

#[allow(dead_code)]
pub fn print_node_child(root: &Node, i: usize) -> usize {
    let mut node_num = i;

    match root {
        Node::Char(c) => {
            println!("\tn{} [label=\"{}\"]", i, if *c == '\0' { "\\0".to_string() } else { c.to_string() });
        }
        Node::Concat((child1, child2)) => {
            println!("\tn{} [label=\"Concat\"]", i);
            let child1_num = print_node_child(child1, node_num + 1);
            println!("\tn{} -> n{}", i, node_num + 1);
            let child2_num = print_node_child(child2, child1_num + 1);
            println!("\tn{} -> n{}", i, child1_num + 1);
            node_num = child2_num;
        }
        Node::Union((child1, child2)) => {
            println!("\tn{} [label=\"Union\"]", i);
            let child1_num = print_node_child(child1, node_num + 1);
            println!("\tn{} -> n{}", i, node_num + 1);
            let child2_num = print_node_child(child2, child1_num + 1);
            println!("\tn{} -> n{}", i, child1_num + 1);
            node_num = child2_num;
        }
        Node::Repeat(child) => {
            println!("\tn{} [label=\"Repeat\"]", i);
            let child_num = print_node_child(child, node_num + 1);
            println!("\tn{} -> n{}", i, node_num + 1);
            node_num = child_num;
        }
        Node::NegChar(set) => {
            let mut s = String::new();
            for c in set {
                s.push(*c);
            }
            println!("\tn{} [label=\"NegChar: {}\"]", i, s);
        }
    };

    node_num + 1
}

#[allow(dead_code)]
pub fn print_node(root: &Node) {
    println!("digraph PARSE {{");
    println!("\tnode [shape=circle]");
    println!("");

    print_node_child(root, 0);

    println!("");
    println!("}}");
}
