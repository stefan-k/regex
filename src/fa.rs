// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq)]
pub struct State(pub Rc<RefCell<RState>>);

// Shouldnt be public!!! fix this (needed in List in main.rs)
#[derive(Debug, Clone, PartialEq)]
pub enum RState {
    RRState { c: Option<char>, out: OutVec },
    Matching,
    NoState,
}

impl Clone for State {
    fn clone(&self) -> Self {
        let State(ref s) = *self;
        return State(Rc::clone(&s));
    }
}

impl State {
    pub fn new_empty() -> Self {
        State(Rc::new(RefCell::new(RState::NoState)))
    }

    pub fn new_char(c: char) -> Self {
        State(Rc::new(RefCell::new(RState::RRState {
            c: Some(c),
            out: OutVec::new(vec![State::new_empty()]),
        })))
    }

    pub fn new_matching() -> Self {
        State(Rc::new(RefCell::new(RState::Matching)))
    }

    pub fn new_split(o0: State, o1: State) -> Self {
        State(Rc::new(RefCell::new(RState::RRState {
            c: None,
            out: OutVec::new(vec![o0, o1]),
        })))
    }

    pub fn clone_out(&self) -> OutVec {
        let State(ref s) = *self;
        match s.borrow().clone() {
            RState::RRState { c: _, out: o } => return o.clone(),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct OutVec(Vec<State>);

impl Clone for OutVec {
    fn clone(&self) -> Self {
        let OutVec(ref o) = *self;
        OutVec::new(o.iter().map(|x| x.clone()).collect())
    }
}

impl OutVec {
    pub fn new(v: Vec<State>) -> Self {
        OutVec(v)
    }

    pub fn attach(&mut self, s: &State) {
        let OutVec(ref mut o) = *self;
        for x in o.iter_mut() {
            let State(ref mut a) = x;
            let State(ref b) = s;
            a.replace(b.borrow().clone());
            // the same as:
            // std::mem::replace(&mut *a.borrow_mut(), b.borrow().clone());
        }
    }
}

pub fn append(o0: &OutVec, o1: &OutVec) -> OutVec {
    let OutVec(ref o0) = *o0;
    let OutVec(ref o1) = *o1;
    let mut o = vec![];
    for oo in o0.iter() {
        o.push(oo.clone());
    }
    for oo in o1.iter() {
        o.push(oo.clone());
    }
    OutVec::new(o)
}
