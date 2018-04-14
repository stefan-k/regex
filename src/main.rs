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
    n: i32,
}

impl List {
    pub fn new() -> Self {
        List { s: vec![], n: 0 }
    }

    pub fn is_match(&self) -> bool {
        self.s
            .iter()
            .filter(|State(x)| x.borrow().clone() == Matching)
            .count() > 0
    }

    pub fn add_state(&mut self, s: &State) -> &mut Self {
        if s.is_split() {
            self.add_state(&s.get_out(0));
            self.add_state(&s.get_out(1));
        } else {
            self.s.push(s.clone());
        }
        self
    }
}

fn fa_match(start: &State, s: String, l1: &mut List, l2: &mut List) -> bool {
    true
}

fn main() {
    // let re = "abb.+.a.".to_owned();
    // let re = "ab.c.".to_owned();
    // let re = "ab.".to_owned();
    let re = "ab|c.".to_owned();
    // let re = "a?".to_owned();
    // let re = "a+".to_owned();
    let start = post2nfa(re);
    println!("{:#?}", start);
    let input = "ab".to_owned();
    let mut l1: List;
    let mut l2: List;
    // let bla = fa_match(&start, input, &mut l1, &mut l2);

    // Simulating the NFA
}
