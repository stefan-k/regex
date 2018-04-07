// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::rc::Rc;

enum Transition {
    Char(char),
    Split,
    Match,
}

struct State {
    c: Transition,
    out: Rc<Option<State>>,
    out1: Rc<Option<State>>,
    lastlist: i32,
}

impl State {
    pub fn new_char(c: char) -> Self {
        State {
            c: Transition::Char(c),
            out: Rc::new(None),
            out1: Rc::new(None),
            lastlist: 0,
        }
    }
}

struct Frag {
    start: Rc<Option<State>>,
    out: Vec<Rc<Option<State>>>,
}

impl Frag {
    pub fn new(start: Rc<Option<State>>, out: Vec<Rc<Option<State>>>) -> Self {
        Frag { start, out }
    }

    pub fn out(&mut self, out: Vec<Rc<Option<State>>>) {
        self.out = out;
    }

    pub fn patch(&mut self, s: &Rc<Option<State>>) {
        self.out = self.out.iter().map(|_| Rc::clone(s)).collect();
    }
}

fn post2nfa(postfix: String) -> Rc<Option<State>> {
    let mut stack: Vec<Frag> = vec![];
    // let stackp: Frag;
    let e: Frag;

    postfix.chars().map(|x| match x {
        '.' => {
            let e2 = stack.pop().unwrap();
            let mut e1 = stack.pop().unwrap();
            e1.patch(&e2.start);
        }
        c => {
            let s = Rc::new(Some(State::new_char(c)));
            stack.push(Frag::new(Rc::clone(&s), vec![Rc::clone(&(s.unwrap().out))]));
        }
    });
    e.start
}

fn main() {
    let re = "abb.+.a.".to_owned();
    println!("Hello, world!");
}
