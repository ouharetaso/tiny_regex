
mod token;
use token::*;
mod parse;
use parse::*;
mod nfa;
use nfa::*;
mod dfa;
use dfa::*;


pub struct TinyRegex {
    dfa: DFA
}

impl TinyRegex {
    pub fn new(regex: &str) -> Result<TinyRegex, String> {
        let mut tokens = tokenize(&regex.to_string())?;
        let root = parse(&mut tokens)?;
        let nfa = build_nfa(root);
        let dfa = DFA::from(nfa);

        Ok(TinyRegex {
            dfa: dfa
        })
    }
    /**
     * @brief returns true iff. there is a match anywhere in the given string
     */
    pub fn is_match(&self, s: &str) -> bool {
        if self.dfa.is_accept(self.dfa.get_start()) {
            return true
        }
        for (i, _) in s.char_indices() {
            let mut state = self.dfa.get_start();
            for c in s[i..].chars() {
                state = self.dfa.transition(c, state);
                if self.dfa.is_accept(state) {
                    return true
                }
            }
        }

        false
    }
}