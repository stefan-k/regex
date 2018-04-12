// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use fa::{append, OutVec, State};

#[derive(Debug)]
struct Frag {
    start: State,
    out: OutVec,
}

impl Frag {
    pub fn new(start: State, out: OutVec) -> Self {
        Frag { start, out }
    }

    pub fn attach(&mut self, s: &State) {
        self.out.attach(s);
    }
}

pub fn post2nfa(postfix: String) -> State {
    let mut stack: Vec<Frag> = vec![];

    for x in postfix.chars() {
        match x {
            '.' => {
                let e2 = stack.pop().unwrap();
                let mut e1 = stack.pop().unwrap();
                e1.attach(&e2.start);
                let mut e = Frag::new(e1.start.clone(), e2.out.clone());
                stack.push(e);
            }
            '|' => {
                let e2 = stack.pop().unwrap();
                let e1 = stack.pop().unwrap();
                let s = State::new_split(e1.start.clone(), e2.start.clone());
                let mut e = Frag::new(s, append(&e1.out, &e2.out));
                stack.push(e);
            }
            '?' => {
                let e1 = stack.pop().unwrap();
                let e2 = State::new_empty();
                let s = State::new_split(e1.start.clone(), e2.clone());
                let mut e = Frag::new(s, append(&e1.out, &OutVec::new(vec![e2.clone()])));
                stack.push(e);
            }
            '*' => {
                let mut e1 = stack.pop().unwrap();
                let e2 = State::new_empty();
                let e3 = e1.start.clone();
                e1.attach(&e3);
                let s = State::new_split(e1.start.clone(), e2.clone());
                let mut e = Frag::new(s.clone(), s.clone_out());
                stack.push(e);
            }
            '+' => {
                let mut e1 = stack.pop().unwrap();
                let e2 = State::new_empty();
                let e3 = e1.start.clone();
                e1.attach(&e3);
                let s = State::new_split(e1.start.clone(), e2.clone());
                let mut e = Frag::new(e1.start.clone(), s.clone_out());
                stack.push(e);
            }
            c => {
                let s = State::new_char(c);
                let o = s.clone_out();
                stack.push(Frag::new(s, o));
            }
        }
    }
    let mut e = stack.pop().unwrap();
    e.attach(&State::new_matching());
    e.start
}
