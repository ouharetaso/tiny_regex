use crate::nfa::*;

use std::collections::HashMap;

const DEAD_STATE: usize = usize::MAX;

struct State {
    transitions: HashMap<char, usize>,
    default_transition: usize,
}

pub struct DFA {
    states: HashMap<usize, State>,
    start: usize,
    accept: Vec<usize>,
    current_state: usize
}


impl State {
    pub fn new(state_num: usize) -> State {
        State {
            transitions: HashMap::new(),
            default_transition: DEAD_STATE,
        }
    }

    pub fn add_transition(&mut self, c: char, state_num: usize) {
        self.transitions.insert(c, state_num);
    }

    pub fn get_transition(&self, c: char) -> Option<&usize> {
        self.transitions.get(&c)
    }

    pub fn set_default_transition(&mut self, state_num: usize) {
        self.default_transition = state_num;
    }
}

impl DFA {
    pub fn new() -> DFA {
        let mut ret = DFA {
            states: HashMap::new(),
            start: 0,
            accept: Vec::new(),
            current_state: 0
        };

        ret.add_state(DEAD_STATE);
        ret.add_default_transition(DEAD_STATE, DEAD_STATE);

        ret
    }

    pub fn add_state(&mut self, state_num: usize) {
        self.states.insert(state_num, State::new(state_num));
    }

    pub fn add_transition(&mut self, state_num: usize, c: char, next_state: usize) {
        self.states.get_mut(&state_num).unwrap().add_transition(c, next_state);
    }

    pub fn add_default_transition(&mut self, state_num: usize, next_state: usize) {
        self.states.get_mut(&state_num).unwrap().set_default_transition(next_state);
    }

    pub fn set_start(&mut self, state_num: usize) {
        self.start = state_num;
    }

    pub fn add_accept(&mut self, state_num: usize) {
        self.accept.push(state_num);
    }

    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn get_accept(&self) -> &Vec<usize> {
        &self.accept
    }

    fn get_state(&self, state_num: usize) -> &State {
        self.states.get(&state_num).unwrap()
    }

    fn get_current_state(&self) -> usize {
        self.current_state
    }

    pub fn transition(&mut self, c: char) {
        let current_state = self.get_current_state();
        let next_state = self.get_state(current_state).get_transition(c);

        if let Some(&state_num) = next_state {
            self.current_state = state_num;
        }
        else {
            self.current_state = self.get_state(current_state).default_transition;
        }
    }

    pub fn is_accept(&self) -> bool {
        self.accept.contains(&self.get_current_state())
    }

    pub fn reset(&mut self) {
        self.current_state = self.start;
    }
}


pub fn print_dfa(dfa: &DFA) {
    println!("digraph DFA {{");
    println!("\tnode [shape=circle]");
    println!("");

    for (state_num, state) in dfa.states.iter() {
        if dfa.accept.contains(state_num) {
            println!("\tn{} [shape=doublecircle]", state_num);
        }
        else {
            println!("\tn{} [shape=circle]", state_num);
        }

        for (c, next_state) in state.transitions.iter() {
            println!("\tn{} -> n{} [label=\"{}\"]", state_num, next_state, c);
        }

        if state.default_transition != DEAD_STATE {
            println!("\tn{} -> n{}", state_num, state.default_transition);
        }
    }

    println!("");
    println!("}}");
}



impl From<NFA> for DFA {
    fn from(nfa: NFA) -> DFA {
        let mut dfa = DFA::new();
        let mut nfa_to_dfa_state_map = HashMap::new();
        let mut dfa_state_num = 0;

        // Get the epsilon closure of the NFA start state
        let nfa_start = nfa.epsilon_closure(nfa.get_start());
        nfa_to_dfa_state_map.insert(nfa_start.clone(), dfa_state_num);
        dfa.add_state(dfa_state_num);
        dfa.set_start(dfa_state_num);

        let mut worklist = vec![nfa_start];
        dfa_state_num += 1;

        // Process each set of NFA states in the worklist
        while !worklist.is_empty() {
            let current_nfa_states = worklist.pop().unwrap();
            let current_dfa_state_num = *nfa_to_dfa_state_map.get(&current_nfa_states).unwrap();

            // For each character transition in the current NFA states
            for c in current_nfa_states.iter().flat_map(|&nfa_state_num| nfa.get_state(nfa_state_num).unwrap().transitions.keys()) {
                let mut next_nfa_states = Vec::new();

                // Collect the next NFA states for the current character
                for &nfa_state_num in current_nfa_states.iter() {
                    if let Some(next_nfa_state_num) = nfa.get_state(nfa_state_num).unwrap().get_transition(*c) {
                        next_nfa_states.extend(nfa.epsilon_closure(*next_nfa_state_num));
                    }
                }

                next_nfa_states.sort();
                next_nfa_states.dedup();

                // If the set of next NFA states is not already mapped to a DFA state
                if !nfa_to_dfa_state_map.contains_key(&next_nfa_states) {
                    nfa_to_dfa_state_map.insert(next_nfa_states.clone(), dfa_state_num);
                    dfa.add_state(dfa_state_num);
                    worklist.push(next_nfa_states.clone());
                    dfa_state_num += 1;
                }

                dfa.add_transition(current_dfa_state_num, *c, *nfa_to_dfa_state_map.get(&next_nfa_states).unwrap());
            }
        }

        // Add accept states to the DFA
        for (nfa_states, dfa_state_num) in nfa_to_dfa_state_map.iter() {
            if nfa_states.contains(&nfa.get_accept()) {
                dfa.add_accept(*dfa_state_num);
            }
        }

        dfa
    }
}