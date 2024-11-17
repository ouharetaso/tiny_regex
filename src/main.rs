#[allow(dead_code)]

mod token;
use token::*;
mod parse;
use parse::*;
mod nfa;
use nfa::*;
mod dfa;
use dfa::*;

#[allow(dead_code)]
fn is_match(dfa: &DFA, s: &String) -> bool {
    let mut state = dfa.get_start();
    for c in s.chars() {
        state = dfa.transition(c, state);
    }

    dfa.is_accept(state)
}

#[allow(dead_code)]
fn print_is_match(dfa: &mut DFA, s: &String) {
    if is_match(dfa, s) {
        println!("{} is a match", s);
    }
    else {
        println!("{} is not a match", s);
    }
}


fn main() -> Result<(), String>{
    let regex = "[a-zA-Z0-9!#]".to_string();
    let mut tokens = tokenize(&regex)?;

    //println!("tokens: {:?}", tokens);

    let root = parse (&mut tokens)?;

    //print_node(&root);

    let nfa = build_nfa(root);

    //print_nfa(&nfa);

    let dfa = DFA::from(nfa);

    print_dfa(&dfa);

    /*
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
    */
    Ok(())
}
