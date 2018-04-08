// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::rc::Rc;

#[derive(Debug)]
enum Transition {
    Char { c: char, out: Option<Rc<State>> },
    Split(Option<Rc<State>>, Option<Rc<State>>),
    Match,
}

impl Transition {
    pub fn out(&self) -> Option<Rc<State>> {
        match *self {
            Transition::Char {
                c: _,
                out: Some(ref out),
            } => Some(Rc::clone(out)),
            Transition::Char { c: _, out: None } => None,
            _ => unimplemented!(),
        }
    }

    // pub fn set_out(self, o: Rc<State>) -> Self {
    //     match self {
    //         Transition::Char { c, out: _ } => Transition::Char { c, out: Some(o) },
    //         _ => unimplemented!(),
    //     }
    // }
}

#[derive(Debug)]
struct State {
    c: Transition,
}

impl State {
    pub fn new_char(c: char, out: Option<Rc<State>>) -> Self {
        State {
            c: Transition::Char { c, out },
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

    pub fn get_char(&self) -> char {
        match self.c {
            Transition::Char { c, out: _ } => c,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct Frag {
    start: Rc<State>,
    // out: Vec<Option<Rc<State>>>,
}

impl Frag {
    pub fn new(start: Rc<State>) -> Self {
        Frag { start }
    }
}

fn post2nfa(postfix: String) -> Rc<State> {
    let mut stack: Vec<Frag> = vec![];

    for x in postfix.chars() {
        match x {
            '.' => {
                let e2 = stack.pop().unwrap();
                let mut e1 = stack.pop().unwrap();
                e1 = Frag::new(Rc::new(State::new_char(
                    e1.start.get_char(),
                    Some(Rc::clone(&e2.start)),
                )));
                stack.push(Frag::new(Rc::clone(&e1.start)));
            }
            c => {
                let s = State::new_char(c, None);
                stack.push(Frag::new(Rc::new(s)));
            }
        }
    }
    stack.pop().unwrap().start
}

fn main() {
    // let re = "abb.+.a.".to_owned();
    let re = "ab.c.".to_owned();
    let bla = post2nfa(re);
    println!("{:?}", bla);
}
