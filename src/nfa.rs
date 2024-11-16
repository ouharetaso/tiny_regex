#[allow(dead_code)]

use std::collections::HashMap;
use crate::parse::*;


const DEAD_STATE: usize = usize::MAX;

pub struct State {
    pub transitions: HashMap<char, usize>,
    pub epsilon_transitions: Vec<usize>,
    pub default_transition: usize,
    pub state_num: usize
}

pub struct NFA {
    states: HashMap<usize, State>,
    start: usize,
    accept: usize
}

#[allow(dead_code)]
impl State {
    pub fn new(state_num: usize) -> State {
        State {
            transitions: HashMap::new(),
            epsilon_transitions: Vec::new(),
            default_transition: DEAD_STATE,
            state_num
        }
    }

    pub fn add_transition(&mut self, c: char, state_num: usize) {
        self.transitions.insert(c, state_num);
    }

    pub fn add_epsilon_transition(&mut self, state_num: usize) {
        self.epsilon_transitions.push(state_num);
        self.epsilon_transitions.sort();
    }

    pub fn get_transition(&self, c: char) -> Option<&usize> {
        self.transitions.get(&c)
    }

    pub fn set_default_transition(&mut self, state_num: usize) {
        self.default_transition = state_num;
    }

    pub fn get_state_num(&self) -> usize {
        self.state_num
    }
}

#[allow(dead_code)]
impl NFA {
    pub fn new() -> NFA {
        NFA {
            states: HashMap::new(),
            start: 0,
            accept: 0
        }
    }

    pub fn epsilon_closure(&self, state_num: usize) -> Vec<usize> {
        let mut closure = Vec::new();
        let mut visited = Vec::<usize>::new();
        let mut stack = Vec::new();
        stack.push(state_num);

        while let Some(state_num) = stack.pop() {
            if visited.contains(&state_num) {
                continue;
            }

            visited.push(state_num);
            closure.push(state_num);

            let state = self.get_state(state_num).unwrap();
            state.epsilon_transitions.iter().for_each(|&next_state_num| {
                stack.push(next_state_num);
            });
        }

        closure.sort();
        closure
    }

    pub fn add_epsilon_transition(&mut self, state_num: usize, next_state_num: usize) {
        self.states.get_mut(&state_num).unwrap().add_epsilon_transition(next_state_num);
    }

    fn add_state(&mut self, state: State) {
        self.states.insert(state.state_num, state);
    }

    fn get_states(&self) -> &HashMap<usize, State> {
        &self.states
    }

    pub fn get_state(&self, state_num: usize) -> Option<&State> {
        self.states.get(&state_num)
    }

    fn get_state_mut(&mut self, state_num: usize) -> Option<&mut State> {
        self.states.get_mut(&state_num)
    }

    pub fn set_start(&mut self, state_num: usize) {
        self.start = state_num;
    }

    pub fn set_accept(&mut self, state_num: usize) {
        self.accept = state_num;
    }

    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn get_accept(&self) -> usize {
        self.accept
    }
}


pub fn build_nfa(root: Node) -> NFA {
    let mut nfa = NFA::new();
    let mut state_num = nfa.get_start();
    state_num = build_nfa_rec(root, &mut nfa, state_num);
    nfa.set_accept(state_num);
    nfa
}


/**
 * @param root: The root of the AST
 * @param nfa: The NFA to build
 * @param state_num: The start state number
 * @return: the accept state number
 */

fn build_nfa_rec(root: Node, nfa: &mut NFA, state_num: usize) -> usize {
    match root {
        Node::Char(c) => {
            let start_state_num = state_num;
            let mut start = State::new(start_state_num);
            let accept_state_num = start_state_num + 1;
            let accept = State::new(accept_state_num);
            start.add_transition(c, accept_state_num);
            nfa.add_state(start);
            nfa.add_state(accept);
            accept_state_num
        }
        Node::Concat((child1, child2)) => {
            let new_start_num = state_num;
            let new_start = State::new(state_num);
            
            let child1_start_num = new_start_num + 1;
            let child1_accept_num = build_nfa_rec(*child1, nfa, child1_start_num);
            
            let child2_start_num = child1_accept_num + 1;
            let child2_accept_num = build_nfa_rec(*child2, nfa, child2_start_num);
            
            let new_accept_num = child2_accept_num + 1;
            let new_accept = State::new(new_accept_num);

            nfa.add_state(new_start);
            nfa.add_state(new_accept);

            nfa.add_epsilon_transition(new_start_num, child1_start_num);
            nfa.add_epsilon_transition(child1_accept_num, child2_start_num);
            nfa.add_epsilon_transition(child2_accept_num, new_accept_num);
                        
            new_accept_num
        }
        Node::Union((child1, child2)) => {
            let new_start_num = state_num;
            let new_start = State::new(state_num);
            
            let child1_start_num = new_start_num + 1;
            let child1_accept_num = build_nfa_rec(*child1, nfa, child1_start_num);
            
            let child2_start_num = child1_accept_num + 1;
            let child2_accept_num = build_nfa_rec(*child2, nfa, child2_start_num);
            
            let new_accept_num = child2_accept_num + 1;
            let new_accept = State::new(new_accept_num);
            
            nfa.add_state(new_start);
            nfa.add_epsilon_transition(new_start_num, child1_start_num);
            nfa.add_epsilon_transition(new_start_num, child2_start_num);

            nfa.add_state(new_accept);
            nfa.add_epsilon_transition(child1_accept_num, new_accept_num);
            nfa.add_epsilon_transition(child2_accept_num, new_accept_num);
            
            new_accept_num
        }
        Node::Repeat(child) => {
            let new_start_num = state_num;
            let new_start = State::new(state_num);
            
            let child_start_num = new_start_num + 1;
            let child_accept_num = build_nfa_rec(*child, nfa, child_start_num);

            let new_accept_num = child_accept_num + 1;
            let new_accept = State::new(new_accept_num);

            nfa.add_state(new_start);
            nfa.add_state(new_accept);

            nfa.add_epsilon_transition(new_start_num, child_start_num);
            nfa.add_epsilon_transition(child_accept_num, child_start_num);
            nfa.add_epsilon_transition(child_accept_num, new_start_num);
            nfa.add_epsilon_transition(new_start_num, new_accept_num);

            new_accept_num
        }
    }
}


pub fn print_nfa(nfa: &NFA) {
    println!("digraph PARSE {{");
    println!("\tnode [shape=circle]");
    println!("");

    nfa.get_states().iter().for_each(|(&_state_num, state)| {
        let state_num = state.get_state_num();
        let transitions = state.transitions.iter();
        let epsilon_transitions = state.epsilon_transitions.iter();
        let default_transition = state.default_transition;

        println!("\tn{} [label=\"n{}\"]", state_num, state_num);

        transitions.for_each(|(&c, next_state_num)| {
            println!("\tn{} -> n{} [label=\"{}\"]", state_num, next_state_num, c);
        });

        epsilon_transitions.for_each(|&next_state_num| {
            println!("\tn{} -> n{} [label=\"ε\"]", state_num, next_state_num);
        });

        if default_transition != DEAD_STATE {
            println!("\tn{} -> n{} [label=\"default\"]", state_num, default_transition);
        }
    });

    println!("");
    println!("}}");
}