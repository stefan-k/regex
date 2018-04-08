// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct State {
    c: Option<char>,
    out: Rc<RefCell<Vec<Option<Rc<RefCell<State>>>>>>,
}

impl State {
    pub fn new_char(c: char) -> Self {
        State {
            c: Some(c),
            out: Rc::new(RefCell::new(vec![None])),
        }
    }

    // pub fn attach(&mut self, s: &Rc<RefCell<State>>) {
    //     for x in self.out.borrow_mut().iter_mut() {
    //         *x = Some(Rc::clone(s));
    //     }
    // }
}

#[derive(Debug)]
struct Frag {
    start: Rc<RefCell<State>>,
    out: Rc<RefCell<Vec<Option<Rc<RefCell<State>>>>>>,
}

impl Frag {
    pub fn new(
        start: Rc<RefCell<State>>,
        out: Rc<RefCell<Vec<Option<Rc<RefCell<State>>>>>>,
    ) -> Self {
        Frag { start, out }
    }

    pub fn attach(&mut self, s: &Rc<RefCell<State>>) {
        for x in self.out.borrow_mut().iter_mut() {
            *x = Some(Rc::clone(s));
        }
    }
}

fn post2nfa(postfix: String) -> Rc<RefCell<State>> {
    let mut stack: Vec<Frag> = vec![];

    for x in postfix.chars() {
        match x {
            '.' => {
                let e2 = stack.pop().unwrap();
                let mut e1 = stack.pop().unwrap();
                e1.attach(&e2.start);
                // e1.start.borrow_mut().attach(&e2.start);
                let mut e = Frag::new(Rc::clone(&e1.start), Rc::clone(&e2.out));
                stack.push(e);
            }
            c => {
                let s = State::new_char(c);
                let o = Rc::clone(&s.out);
                stack.push(Frag::new(Rc::new(RefCell::new(s)), o));
            }
        }
    }
    println!("{:#?}", stack);
    stack.pop().unwrap().start
}

fn main() {
    // let re = "abb.+.a.".to_owned();
    let re = "ab.c.".to_owned();
    // let re = "ab.".to_owned();
    let bla = post2nfa(re);
    // println!("{:?}", bla);
}
