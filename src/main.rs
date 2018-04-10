// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct State(Rc<RefCell<RState>>);

#[derive(Debug, Clone)]
struct RState {
    c: Option<char>,
    out: OutVec,
}

impl Clone for State {
    fn clone(&self) -> Self {
        let State(ref s) = *self;
        State(Rc::clone(&s))
    }
}

impl State {
    pub fn new_char(c: char) -> Self {
        State(Rc::new(RefCell::new(RState {
            c: Some(c),
            out: OutVec::new(vec![None]),
        })))
    }

    pub fn new_split(o0: State, o1: State) -> Self {
        State(Rc::new(RefCell::new(RState {
            c: None,
            out: OutVec::new(vec![Some(o0), Some(o1)]),
        })))
    }

    pub fn clone_out(&self) -> OutVec {
        let State(ref s) = *self;
        s.borrow_mut().out.clone()
    }
}

#[derive(Debug)]
struct OutVec(Rc<RefCell<Vec<Option<State>>>>);

impl Clone for OutVec {
    fn clone(&self) -> Self {
        let OutVec(ref o) = *self;
        OutVec(Rc::clone(o))
    }
}

impl OutVec {
    pub fn new(v: Vec<Option<State>>) -> Self {
        OutVec(Rc::new(RefCell::new(v)))
    }

    pub fn attach(&mut self, s: &State) {
        let OutVec(ref mut o) = *self;
        for x in o.borrow_mut().iter_mut() {
            *x = Some(s.clone());
        }
    }
}

fn append(o0: &OutVec, o1: &OutVec) -> OutVec {
    let OutVec(ref o0) = *o0;
    let OutVec(ref o1) = *o1;
    let mut o = vec![];
    for oo in o0.borrow().iter() {
        o.push(oo.clone());
    }
    for oo in o1.borrow().iter() {
        o.push(oo.clone());
    }
    OutVec::new(o)
}

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

fn post2nfa(postfix: String) -> State {
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
            c => {
                let s = State::new_char(c);
                let o = s.clone_out();
                stack.push(Frag::new(s, o));
            }
        }
    }
    println!("{:#?}", stack);
    stack.pop().unwrap().start
}

fn main() {
    // let re = "abb.+.a.".to_owned();
    // let re = "ab.c.".to_owned();
    // let re = "ab.".to_owned();
    let re = "ab|".to_owned();
    let bla = post2nfa(re);
    // println!("{:?}", bla);
}
