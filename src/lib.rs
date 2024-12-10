
mod token;
use token::*;
mod parse;
use parse::*;
mod nfa;
use nfa::*;
mod dfa;
use dfa::*;

use std::collections::VecDeque;

#[cfg(feature = "on_the_fly")]
pub type TinyRegex = TinyRegexInner<OnTheFlyDFA>;

#[cfg(not(feature = "on_the_fly"))]
pub type TinyRegex = TinyRegexInner<DFA>;

pub struct TinyRegexInner<T: DFAExt> {
    dfa: T
}

#[derive(PartialEq, Debug, Clone)]
pub struct Match<'a> {
    start: usize,
    end: usize,
    matched_str: &'a str
}

impl<T: DFAExt> TinyRegexInner<T> {
    pub fn new(regex: &str) -> Result<TinyRegexInner<T>, String> {
        let mut tokens = tokenize(&regex.to_string())?;
        let root = parse(&mut tokens)?;
        let nfa = build_nfa(root);
        let dfa = T::new(nfa);

        Ok(TinyRegexInner {
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

    pub fn find_at<'a>(&self, s: &'a str, start: usize) -> Option<Match<'a>> {
        if self.dfa.is_accept(self.dfa.get_start()) {
            return Some(Match {
                start: start,
                end: start,
                matched_str: &s[start..start]
            })
        }

        for (i, _) in s[start..].char_indices() {
            let mut state = self.dfa.get_start();
            let mut is_match = false;
            let mut end  = i;
            for (j, c) in s[i+start..].char_indices() {
                state = self.dfa.transition(c, state);
                if self.dfa.is_accept(state) {
                    is_match = true;
                    let mut chars = s[start+i+j..].char_indices();
                    chars.next();
                    end = if let Some((e,_)) = chars.next() {
                        i+j+e
                    }
                    else {
                        s.len() - start
                    }
                }
                else if self.dfa.is_dead(state) {
                    break;
                }
            }
            if is_match {
                return Some(Match {
                    start: start + i,
                    end: start + end,
                    matched_str: &s[start + i..start + end]
                });
            }
        }
        None
    } 

    pub fn find_all<'a>(&self, s: &'a str) -> Matches<'a> {
        let mut matches = VecDeque::<Match>::new();
        let mut i = 0;

        while let Some(mat) = self.find_at(s, i) {
            i = mat.end();
            matches.push_back(mat);
        }

        Matches::new(matches)
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
        self.end - self.start
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

#[derive(Clone)]
pub struct Matches<'a> {
    matches: VecDeque<Match<'a>>
}


impl<'a> Iterator for Matches<'a> {
    type Item = Match<'a>;

    fn next(&mut self) -> Option<Match<'a>> {
        self.matches.pop_front()
    }
}

impl<'a> Matches<'a> {
    fn new(matches: VecDeque<Match<'a>>) -> Matches<'a> {
        Matches {
            matches
        }
    }
}



#[cfg(test)]
#[cfg(not(feature = "on_the_fly"))]
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

    #[test]
    fn test_for_readme() {
        let re = TinyRegex::new("a(b|c)*d").unwrap();
        assert!(re.is_match("abbbcd"));
        assert!(!re.is_match("abbbce"));

        let mat = re.find("wxyzabbbcdeffe").unwrap();
        assert_eq!(mat.start(), 4);
        assert_eq!(mat.end(), 10);
        assert_eq!(mat.as_str(), "abbbcd");
        assert_eq!(mat.range(), 4..10);
    }

    #[test]
    fn test_find_all() {
        let re = TinyRegex::new("[a-zA-Z][a-zA-Z]*").unwrap();
        let s = "my name is Unyo";

        let mut matches = re.find_all(s);
        assert_eq!(matches.next().unwrap().as_str(), "my");
        assert_eq!(matches.next().unwrap().as_str(), "name");
        assert_eq!(matches.next().unwrap().as_str(), "is");
        assert_eq!(matches.next().unwrap().as_str(), "Unyo");
        assert_eq!(matches.next(), None);
    }

    #[test]
    fn test_negchar() {
        let re = TinyRegex::new("[^・ー]").unwrap();
        let s = "エドワード・ノートン\n";

        let mut matches = re.find_all(s);
        assert_eq!(matches.next().unwrap().as_str(), "エ");
        assert_eq!(matches.next().unwrap().as_str(), "ド");
        assert_eq!(matches.next().unwrap().as_str(), "ワ");
        assert_eq!(matches.next().unwrap().as_str(), "ド");
        assert_eq!(matches.next().unwrap().as_str(), "ノ");
        assert_eq!(matches.next().unwrap().as_str(), "ト");
        assert_eq!(matches.next().unwrap().as_str(), "ン");
        assert_eq!(matches.next().unwrap().as_str(), "\n");
        assert_eq!(matches.next(), None);
    }
}

#[cfg(test)]
#[cfg(feature = "on_the_fly")]
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

    #[test]
    fn test_for_readme() {
        let re = TinyRegex::new("a(b|c)*d").unwrap();
        assert!(re.is_match("abbbcd"));
        assert!(!re.is_match("abbbce"));

        let mat = re.find("wxyzabbbcdeffe").unwrap();
        assert_eq!(mat.start(), 4);
        assert_eq!(mat.end(), 10);
        assert_eq!(mat.as_str(), "abbbcd");
        assert_eq!(mat.range(), 4..10);
    }

    #[test]
    fn test_find_all() {
        let re = TinyRegex::new("[a-zA-Z][a-zA-Z]*").unwrap();
        let s = "my name is Unyo";

        let mut matches = re.find_all(s);
        assert_eq!(matches.next().unwrap().as_str(), "my");
        assert_eq!(matches.next().unwrap().as_str(), "name");
        assert_eq!(matches.next().unwrap().as_str(), "is");
        assert_eq!(matches.next().unwrap().as_str(), "Unyo");
        assert_eq!(matches.next(), None);
    }

    #[test]
    fn test_negchar() {
        let re = TinyRegex::new("[^・ー]").unwrap();
        let s = "エドワード・ノートン\n";

        let mut matches = re.find_all(s);
        assert_eq!(matches.next().unwrap().as_str(), "エ");
        assert_eq!(matches.next().unwrap().as_str(), "ド");
        assert_eq!(matches.next().unwrap().as_str(), "ワ");
        assert_eq!(matches.next().unwrap().as_str(), "ド");
        assert_eq!(matches.next().unwrap().as_str(), "ノ");
        assert_eq!(matches.next().unwrap().as_str(), "ト");
        assert_eq!(matches.next().unwrap().as_str(), "ン");
        assert_eq!(matches.next().unwrap().as_str(), "\n");
        assert_eq!(matches.next(), None);
    }
}