// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::rc::Rc;

#[derive(Debug)]
enum Transition {
    Char { c: char, out: Rc<Option<State>> },
    Split(Option<Rc<State>>, Option<Rc<State>>),
    Match,
}

impl Transition {
    pub fn out(&self) -> Rc<Option<State>> {
        match *self {
            Transition::Char { c: _, ref out } => Rc::clone(out),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct State {
    c: Transition,
}

impl State {
    pub fn new_char(c: char) -> Self {
        State {
            c: Transition::Char {
                c: c,
                out: Rc::new(None),
            },
        }
    }

    pub fn new_split() -> Self {
        State {
            c: Transition::Split(None, None),
        }
    }

    pub fn new_match() -> Self {
        State {
            c: Transition::Match,
        }
    }
}

#[derive(Debug)]
struct Frag {
    start: Rc<State>,
    out: Vec<Rc<Option<State>>>,
}

impl Frag {
    pub fn new(start: Rc<State>, out: Vec<Rc<Option<State>>>) -> Self {
        Frag { start, out }
    }
}

// fn post2nfa(postfix: String) -> Rc<State> {
fn post2nfa(postfix: String) -> Vec<Frag> {
    let mut stack: Vec<Frag> = vec![];
    // let stackp: Frag;
    // let e: Frag;

    for x in postfix.chars() {
        match x {
            c => {
                let s = State::new_char(c);
                let list = vec![s.c.out()];
                stack.push(Frag::new(Rc::new(s), list));
            }
        }
    }
    // postfix
    //     .chars()
    //     .map(|x| match x {
    //     // '.' => {
    //     //     let e2 = stack.pop();
    //     //     let mut e1 = stack.pop();
    //     //     // e1.patch(&e2.start);
    //     // }
    //     c => {
    //         let s = State::new_char(c);
    //         let list = vec![s.c.out()];
    //         stack.push(Frag::new(Rc::new(s), list));
    //     }
    //     // _ => unimplemented!(),
    // })
    //     .collect();
    // e.start
    stack
}

fn main() {
    // let re = "abb.+.a.".to_owned();
    let re = "abb".to_owned();
    let bla = post2nfa(re);
    println!("{:?}", bla);
}
