use crate::nfa::*;

use std::collections::HashMap;
use std::cell::RefCell;

pub const DEAD_STATE: usize = usize::MAX;

struct State {
    transitions: HashMap<char, usize>,
    default_transition: usize,
}

pub struct DFA {
    states: HashMap<usize, State>,
    start: usize,
    accept: Vec<usize>,
}

pub trait DFAExt {
    fn new(nfa: NFA) -> Self;
    fn is_accept(&self, state: usize) -> bool;
    fn is_dead(&self, state: usize) -> bool;
    fn transition(&self, c: char, current_state: usize) -> usize;
    fn get_start(&self) -> usize;
}


impl DFAExt for DFA {
    fn new(nfa: NFA) -> Self {
        From::from(nfa)
    }

    fn is_accept(&self, state: usize) -> bool {
        self.accept.contains(&state)
    }

    fn is_dead(&self, state: usize) -> bool {
        state == DEAD_STATE
    }

    fn transition(&self, c: char, current_state: usize) -> usize {
        let next_state = self.get_state(current_state).get_transition(c);

        if let Some(&state_num) = next_state {
            state_num
        }
        else {
            self.get_state(current_state).default_transition
        }
    }

    fn get_start(&self) -> usize {
        self.start
    }
}


impl State {
    pub fn new() -> State {
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
    pub fn add_state(&mut self, state_num: usize) {
        self.states.insert(state_num, State::new());
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

    fn get_state(&self, state_num: usize) -> &State {
        self.states.get(&state_num).unwrap()
    }

    fn state_num_range(&self) -> std::ops::Range<usize> {
        0..self.states.len()
    }

}

#[allow(dead_code)]
pub fn print_dfa(dfa: &DFA) {
    println!("digraph DFA {{");
    println!("\tnode [shape=circle]");
    println!("");

    for (state_num, state) in dfa.states.iter() {
        if dfa.accept.contains(state_num) {
            println!("\tn{} [shape=doublecircle]", state_num);
        }
        else if state_num == &DEAD_STATE {
            ()
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

pub struct OnTheFlyDFA {
    nfa: NFA,
    dfa: RefCell<DFA>,
    nfa_to_dfa_state_map: RefCell<HashMap<Vec<usize>, usize>>,
}

impl DFAExt for OnTheFlyDFA {
    fn new(nfa: NFA) -> Self {
        let mut dfa = DFA{
            states: HashMap::new(),
            start: 0,
            accept: Vec::new(),
        };

        dfa.add_state(DEAD_STATE);
        dfa.add_default_transition(DEAD_STATE, DEAD_STATE);

        let mut nfa_to_dfa_state_map = HashMap::new();

        let dfa_start_num = 0;

        // Get the epsilon closure of the NFA start state
        let nfa_start = nfa.epsilon_closure(nfa.get_start());
        nfa_to_dfa_state_map.insert(nfa_start.clone(), dfa_start_num);
        nfa_to_dfa_state_map.insert(vec![DEAD_STATE], DEAD_STATE);
        dfa.add_state(dfa_start_num);
        dfa.set_start(dfa_start_num);

        OnTheFlyDFA {
            nfa,
            dfa: RefCell::new(dfa),
            nfa_to_dfa_state_map: RefCell::new(nfa_to_dfa_state_map),
        }
    }

    fn transition(&self, c: char, current_state: usize) -> usize {
        let current_dfa_state_num = current_state;
        let mut dfa = self.dfa.borrow_mut();

        if dfa.is_dead(current_dfa_state_num) {
            return DEAD_STATE;
        }
        // already visited
        else if let Some(&next_state) = dfa.get_state(current_dfa_state_num).get_transition(c) {
            return next_state;
        }

        let next_dfa_state_num;
        let mut nfa_to_dfa_state_map = self.nfa_to_dfa_state_map.borrow_mut();
        let current_nfa_states = nfa_to_dfa_state_map.iter().find(|(_, &state_num)| state_num == current_dfa_state_num).unwrap().0;
        let mut next_nfa_states = Vec::new();

        for &nfa_state_num in current_nfa_states.iter() {
            let nfa_state = self.nfa.get_state(nfa_state_num).unwrap();

            if let Some(next_nfa_state_num) = nfa_state.get_transition(c) {
                next_nfa_states.extend(self.nfa.epsilon_closure(*next_nfa_state_num));
            }
            else {
                next_nfa_states.extend(self.nfa.epsilon_closure(nfa_state.default_transition));
            }
        }

        next_nfa_states.sort();
        next_nfa_states.dedup();

        if !nfa_to_dfa_state_map.contains_key(&next_nfa_states) {
            next_dfa_state_num = dfa.state_num_range().end;
            nfa_to_dfa_state_map.insert(next_nfa_states.clone(), next_dfa_state_num);
            dfa.add_state(next_dfa_state_num);
            dfa.add_transition(current_dfa_state_num, c, next_dfa_state_num);
        }
        else {
            next_dfa_state_num = *nfa_to_dfa_state_map.get(&next_nfa_states).unwrap();
            dfa.add_transition(current_dfa_state_num, c, next_dfa_state_num);
        }

        if next_nfa_states.contains(&self.nfa.get_accept()) {
            dfa.add_accept(next_dfa_state_num);
        }

        next_dfa_state_num
    }

    fn is_accept(&self, state: usize) -> bool {
        self.dfa.borrow().is_accept(state)
    }

    fn is_dead(&self, state: usize) -> bool {
        self.dfa.borrow().is_dead(state)
    }

    fn get_start(&self) -> usize {
        self.dfa.borrow().get_start()
    }
}

impl From<NFA> for DFA {
    fn from(nfa: NFA) -> DFA {
        let mut dfa = DFA{
            states: HashMap::new(),
            start: 0,
            accept: Vec::new(),
        };

        dfa.add_state(DEAD_STATE);
        dfa.add_default_transition(DEAD_STATE, DEAD_STATE);

        let mut nfa_to_dfa_state_map = HashMap::new();
        let mut dfa_state_num = 0;

        // Get the epsilon closure of the NFA start state
        let nfa_start = nfa.epsilon_closure(nfa.get_start());
        nfa_to_dfa_state_map.insert(nfa_start.clone(), dfa_state_num);
        nfa_to_dfa_state_map.insert(vec![DEAD_STATE], DEAD_STATE);
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
                    let nfa_state = nfa.get_state(nfa_state_num).unwrap();
                    if let Some(next_nfa_state_num) = nfa_state.get_transition(*c) {
                        next_nfa_states.extend(nfa.epsilon_closure(*next_nfa_state_num));
                    }
                    else {
                        next_nfa_states.extend(nfa.epsilon_closure(nfa_state.default_transition));
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

            // treat default transitions
            let mut next_nfa_states = Vec::new();

            // Collect the next NFA states for the current character
            for &nfa_state_num in current_nfa_states.iter() {
                next_nfa_states.extend(nfa.epsilon_closure(nfa.get_state(nfa_state_num).unwrap().default_transition));
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

            dfa.add_default_transition(current_dfa_state_num, *nfa_to_dfa_state_map.get(&next_nfa_states).unwrap());
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;
    use crate::parse::*;

    #[test]
    fn count_states() {
        let regex = "[a-zA-Z0-9]".repeat(30);
        let mut tokens = tokenize(&regex.to_string()).unwrap();
        let root = parse(&mut tokens).unwrap();
        let nfa = build_nfa(root);
        let dfa = DFA::new(nfa);

        assert_eq!(dfa.states.len(), 1862);
    }
}