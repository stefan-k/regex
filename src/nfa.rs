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
