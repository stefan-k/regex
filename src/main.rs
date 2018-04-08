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
    out: RefCell<Vec<Option<RefCell<State>>>>,
}

impl State {
    pub fn new_char(c: char) -> Self {
        State {
            c: Some(c),
            out: RefCell::new(vec![None]),
        }
    }

    pub fn attach(&mut self, s: &RefCell<State>) {
        for x in self.out.borrow_mut().iter_mut() {
            *x = Some(RefCell::clone(s));
        }
    }
}

#[derive(Debug)]
struct Frag {
    start: RefCell<State>,
    out: RefCell<Vec<Option<RefCell<State>>>>,
}

impl Frag {
    pub fn new(start: RefCell<State>, out: RefCell<Vec<Option<RefCell<State>>>>) -> Self {
        Frag { start, out }
    }

    pub fn attach(&mut self, s: &RefCell<State>) {
        for x in self.out.borrow_mut().iter_mut() {
            *x = Some(RefCell::clone(s));
        }
    }
}

fn post2nfa(postfix: String) -> RefCell<State> {
    let mut stack: Vec<Frag> = vec![];

    for x in postfix.chars() {
        match x {
            '.' => {
                let e2 = stack.pop().unwrap();
                let mut e1 = stack.pop().unwrap();
                println!("fu1: {:?}", e1);
                e1.attach(&e2.start);
                e1.start.borrow_mut().attach(&e2.start);
                println!("fu2: {:?}", e1);
                let e = Frag::new(RefCell::clone(&e1.start), RefCell::clone(&e2.out));
                // println!("fu3: {:?}", e);
                stack.push(e);
            }
            c => {
                let s = State::new_char(c);
                let o = RefCell::clone(&s.out);
                stack.push(Frag::new(RefCell::new(s), o));
            }
        }
        // println!("bla: {}", x);
        println!("{:#?}", stack);
    }
    println!("{:?}", stack);
    stack.pop().unwrap().start
}

fn main() {
    // let re = "abb.+.a.".to_owned();
    // let re = "ab.c.".to_owned();
    let re = "ab.".to_owned();
    let bla = post2nfa(re);
    println!("{:?}", bla);
}
