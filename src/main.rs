// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

mod fa;
mod nfa;

use nfa::{fa_match, post2nfa};

fn main() {
    // let re = "abb.+.a.".to_owned();
    // let re = "ab.c.".to_owned();
    // let re = "ab.".to_owned();
    // let re = "ab|c.".to_owned();
    // let re = "a?".to_owned();
    // let re = "a+".to_owned();
    let re = "a*".to_owned();
    let start = post2nfa(re);
    // println!("{:#?}", start);
    let input = "".to_owned();
    // let input = "abbbba".to_owned();
    let bla = fa_match(&start, input);
    println!("{:#?}", bla);

    // Simulating the NFA
}
