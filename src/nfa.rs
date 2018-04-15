// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std;
use fa::{append, OutVec, State};
use fa::RState::Matching;

#[derive(Debug)]
struct Frag {
    start: State,
    out: OutVec,
}

impl Frag {
    pub fn new(start: &State, out: &OutVec) -> Self {
        Frag {
            start: start.clone(),
            out: out.clone(),
        }
    }

    pub fn attach(&mut self, s: &State) {
        self.out.attach(s);
    }
}

pub fn post2nfa(postfix: String) -> State {
    let mut stack: Vec<Frag> = vec![];

    for x in postfix.chars() {
        match x {
            // catenation
            '.' => {
                let e2 = stack.pop().unwrap();
                let mut e1 = stack.pop().unwrap();
                e1.attach(&e2.start);
                let mut e = Frag::new(&e1.start, &e2.out);
                stack.push(e);
            }
            // alternation
            '|' => {
                let e2 = stack.pop().unwrap();
                let e1 = stack.pop().unwrap();
                let s = State::new_split(&e1.start, &e2.start);
                let mut e = Frag::new(&s, &append(&e1.out, &e2.out));
                stack.push(e);
            }
            // zero or one
            '?' => {
                let e1 = stack.pop().unwrap();
                let e2 = State::new_empty();
                let s = State::new_split(&e1.start, &e2);
                let mut e = Frag::new(&s, &append(&e1.out, &OutVec::new(vec![e2.clone()])));
                stack.push(e);
            }
            // zero or more
            '*' => {
                let mut e1 = stack.pop().unwrap();
                let e2 = State::new_empty();
                let s = State::new_split(&e1.start, &e2);
                e1.attach(&s);
                let mut e = Frag::new(&s, &s.clone_out());
                stack.push(e);
            }
            // one or more
            '+' => {
                let mut e1 = stack.pop().unwrap();
                let s2 = State::new_empty();
                let s = State::new_split(&e1.start, &s2);
                e1.attach(&s);
                let mut e = Frag::new(&e1.start, &s.clone_out());
                stack.push(e);
            }
            // character
            c => {
                let s = State::new_char(c);
                stack.push(Frag::new(&s, &s.clone_out()));
            }
        }
    }
    let mut e = stack.pop().unwrap();
    e.attach(&State::new_matching());
    e.start
}

struct List {
    s: Vec<State>,
}

impl List {
    pub fn new() -> Self {
        List { s: vec![] }
    }

    pub fn is_match(&self) -> bool {
        self.s
            .iter()
            .filter(|State(x)| x.borrow().clone() == Matching)
            .count() > 0
    }

    pub fn add_start(&mut self, start: &State) -> &mut Self {
        self.add_state(&OutVec::new(vec![start.clone()]))
    }

    pub fn add_state(&mut self, o: &OutVec) -> &mut Self {
        let OutVec(ref o) = *o;
        for s in o.iter() {
            if s.is_split() {
                self.add_state(&s.clone_out());
            } else {
                self.s.push(s.clone());
            }
        }
        self
    }

    pub fn clear(&mut self) {
        self.s.clear();
    }
}

pub fn fa_match(start: &State, s: String) -> bool {
    let mut clist = List::new();
    let mut nlist = List::new();
    clist.add_start(start);
    for c in s.chars() {
        nlist.clear();
        for st in clist.s.iter() {
            if !st.is_matching() {
                if st.is_split() {
                    nlist.add_start(&st);
                } else if st.get_char() == c {
                    nlist.add_state(&st.clone_out());
                }
            }
        }
        std::mem::swap(&mut clist, &mut nlist);
    }
    clist.is_match()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catenation() {
        let re = "ab.".to_owned();
        let start = post2nfa(re);
        assert_eq!(true, fa_match(&start, "ab".to_owned()));
        assert_eq!(false, fa_match(&start, "ba".to_owned()));
        assert_eq!(false, fa_match(&start, "a".to_owned()));
        assert_eq!(false, fa_match(&start, "abab".to_owned()));
        assert_eq!(false, fa_match(&start, "".to_owned()));
    }

    #[test]
    fn alternation() {
        let re = "ab|".to_owned();
        let start = post2nfa(re);
        assert_eq!(true, fa_match(&start, "a".to_owned()));
        assert_eq!(true, fa_match(&start, "b".to_owned()));
        assert_eq!(false, fa_match(&start, "".to_owned()));
        assert_eq!(false, fa_match(&start, "ba".to_owned()));
        assert_eq!(false, fa_match(&start, "c".to_owned()));
        assert_eq!(false, fa_match(&start, "bb".to_owned()));
        assert_eq!(false, fa_match(&start, "bvccb".to_owned()));
    }

    #[test]
    fn zero_or_one() {
        let re = "a?".to_owned();
        let start = post2nfa(re);
        assert_eq!(true, fa_match(&start, "a".to_owned()));
        assert_eq!(true, fa_match(&start, "".to_owned()));
        assert_eq!(false, fa_match(&start, "aaaa".to_owned()));
        assert_eq!(false, fa_match(&start, "c".to_owned()));
        assert_eq!(false, fa_match(&start, "bb".to_owned()));
        assert_eq!(false, fa_match(&start, "bvccb".to_owned()));
    }

    #[test]
    fn zero_or_more() {
        let re = "a*".to_owned();
        let start = post2nfa(re);
        assert_eq!(true, fa_match(&start, "a".to_owned()));
        assert_eq!(true, fa_match(&start, "".to_owned()));
        assert_eq!(true, fa_match(&start, "aaaa".to_owned()));
        assert_eq!(false, fa_match(&start, "c".to_owned()));
        assert_eq!(false, fa_match(&start, "bb".to_owned()));
        assert_eq!(false, fa_match(&start, "bvccb".to_owned()));
    }

    #[test]
    fn one_or_more() {
        let re = "a+".to_owned();
        let start = post2nfa(re);
        assert_eq!(true, fa_match(&start, "a".to_owned()));
        assert_eq!(true, fa_match(&start, "aaaa".to_owned()));
        assert_eq!(false, fa_match(&start, "".to_owned()));
        assert_eq!(false, fa_match(&start, "c".to_owned()));
        assert_eq!(false, fa_match(&start, "bb".to_owned()));
        assert_eq!(false, fa_match(&start, "bvccb".to_owned()));
    }
}
