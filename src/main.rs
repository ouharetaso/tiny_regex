#[allow(dead_code)]

mod token;
use token::*;
mod parse;
use parse::*;
mod nfa;
use nfa::*;
mod dfa;
use dfa::*;


fn print_node_child(root: &Node, i: usize) -> usize {
    let mut node_num = i;

    match root {
        Node::Char(c) => {
            println!("\tn{} [label=\"{}\"]", i, c);
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
    };

    node_num + 1
}


fn print_node(root: &Node) {
    println!("digraph PARSE {{");
    println!("\tnode [shape=circle]");
    println!("");

    print_node_child(root, 0);

    println!("");
    println!("}}");
}


fn is_match(dfa: &mut DFA, s: &String) -> bool {
    for c in s.chars() {
        dfa.transition(c);
    }

    let ret = dfa.is_accept();
    dfa.reset();

    ret
}

fn print_is_match(dfa: &mut DFA, s: &String) {
    if is_match(dfa, s) {
        println!("{} is a match", s);
    }
    else {
        println!("{} is not a match", s);
    }
}


fn main() -> Result<(), String>{
    let regex = "a*(b|cd*)".to_string();
    let mut tokens = tokenize(&regex)?;
    let root = parse (&mut tokens)?;

    //print_node(&root);

    let nfa = build_nfa(root);

    //print_nfa(&nfa);

    let mut dfa = DFA::from(nfa);

    print_is_match(&mut dfa, &"a".to_string());
    print_is_match(&mut dfa, &"aa".to_string());
    print_is_match(&mut dfa, &"b".to_string());
    print_is_match(&mut dfa, &"c".to_string());
    print_is_match(&mut dfa, &"cd".to_string());
    print_is_match(&mut dfa, &"cdd".to_string());
    print_is_match(&mut dfa, &"cddd".to_string());
    print_is_match(&mut dfa, &"cc".to_string());
    print_is_match(&mut dfa, &"ccdd".to_string());
    print_is_match(&mut dfa, &"ccddd".to_string());
    print_is_match(&mut dfa, &"abcd".to_string());
    print_is_match(&mut dfa, &"ccdddd".to_string());

    Ok(())
}
