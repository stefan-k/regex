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
    Char { c: Option<char>, out: OutVec },
    Split { out: OutVec },
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
        State(Rc::new(RefCell::new(RState::Char {
            c: Some(c),
            out: OutVec::new(vec![State::new_empty()]),
        })))
    }

    pub fn new_matching() -> Self {
        State(Rc::new(RefCell::new(RState::Matching)))
    }

    pub fn new_split(o0: &State, o1: &State) -> Self {
        State(Rc::new(RefCell::new(RState::Split {
            out: OutVec::new(vec![o0.clone(), o1.clone()]),
        })))
    }

    pub fn clone_out(&self) -> OutVec {
        let State(ref s) = *self;
        match s.borrow().clone() {
            RState::Char { c: _, out: o } => return o.clone(),
            RState::Split { out: o } => return o.clone(),
            _ => unimplemented!(),
        }
    }

    // pub fn get_out(&self, idx: usize) -> State {
    //     let State(ref s) = *self;
    //     match s.borrow().clone() {
    //         RState::Char { c: _, out: o } => return o.get(0),
    //         RState::Split { out: o } => return o.get(idx),
    //         _ => unimplemented!(),
    //     }
    // }

    pub fn get_char(&self) -> char {
        let State(ref s) = *self;
        match s.borrow().clone() {
            RState::Char { c: Some(c), out: _ } => return c,
            _ => unimplemented!(),
        }
    }

    pub fn is_split(&self) -> bool {
        let State(ref s) = self;
        // Is there a fancier way to do this?
        if let RState::Split { out: _ } = s.borrow().clone() {
            true
        } else {
            false
        }
    }

    pub fn is_matching(&self) -> bool {
        let State(ref s) = self;
        match s.borrow().clone() {
            RState::Matching => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct OutVec(pub Vec<State>);

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

    // pub fn get(&self, i: usize) -> State {
    //     let OutVec(ref o) = *self;
    //     o[i].clone()
    // }

    pub fn attach(&mut self, s: &State) {
        let OutVec(ref mut o) = *self;
        for x in o.iter_mut() {
            let State(ref mut a) = x;
            let State(ref b) = s;
            if a.borrow().clone() == RState::NoState {
                a.replace(b.borrow().clone());
                // the same as:
                // std::mem::replace(&mut *a.borrow_mut(), b.borrow().clone());
            }
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
