// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod fa;
mod nfa;

use nfa::post2nfa;
use fa::{OutVec, State};
use fa::RState::Matching;

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

fn fa_match(start: &State, s: String) -> bool {
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

fn main() {
    // let re = "abb.+.a.".to_owned();
    // let re = "ab.c.".to_owned();
    // let re = "ab.".to_owned();
    // let re = "ab|c.".to_owned();
    // let re = "a?".to_owned();
    // let re = "a+".to_owned();
    let re = "a*".to_owned();
    let start = post2nfa(re);
    // println!("{:#?}", start);
    let input = "".to_owned();
    // let input = "abbbba".to_owned();
    let bla = fa_match(&start, input);
    println!("{:#?}", bla);

    // Simulating the NFA
}
