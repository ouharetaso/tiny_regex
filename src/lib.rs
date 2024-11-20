
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

pub struct Match<'a> {
    start: usize,
    end: usize,
    matched_str: &'a str
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

    pub fn find<'a>(&self, s: &'a str) -> Option<Match<'a>> {
        if self.dfa.is_accept(self.dfa.get_start()) {
            return Some(Match {
                start: 0,
                end: 0,
                matched_str: &s[0..0]
            })
        }

        for (i, _) in s.char_indices() {
            let mut state = self.dfa.get_start();
            let mut is_match = false;
            let mut end  = i;
            for (j, c) in s[i..].char_indices() {
                state = self.dfa.transition(c, state);
                if self.dfa.is_accept(state) {
                    is_match = true;
                    let mut chars = s[i+j..].char_indices();
                    chars.next();
                    end = if let Some((e,_)) = chars.next() {
                        i+j+e
                    }
                    else {
                        s.len()
                    }
                }
                else if self.dfa.is_dead(state) {
                    break;
                }
            }
            if is_match {
                return Some(Match {
                    start: i,
                    end: end,
                    matched_str: &s[i..end]
                });
            }
        }

        None
    }
}

impl<'a> Match<'a> {
    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn len(&self) -> usize {
        self.end - self.start + 1
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn as_str(&self) -> &'a str {
        self.matched_str
    }

    pub fn range(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }
}



#[cfg(test)]
mod tests {
    use crate::TinyRegex;

    #[test]
    fn test_find() {
        let re = TinyRegex::new("[a-zA-Z][a-zA-Z0-9]*").unwrap();
        let s = "うにょ hello114514";
        let mat = re.find(s).unwrap();

        assert_eq!(10..21, mat.range());
        assert_eq!("hello114514", &s[mat.range()]);
        assert_eq!("hello114514", mat.as_str());
    }
}