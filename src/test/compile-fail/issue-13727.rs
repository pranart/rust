// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn test(val: u8) {
  match val {
    256 => print!("0b1110\n"),
    512 => print!("0b1111\n"),
    //~^ ERROR: unreachable pattern
    _   => print!("fail\n"),
  }
}

fn main() {
  test(1);
}