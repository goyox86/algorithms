// Copyright 2014 Jose Narvaez. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*!

Implementation of "Quick Find" algorithm for the "Dynamic Conectivity"
problem of lecture 1.

*/

struct UF {
  vec: Vec<uint>
}

impl UF {
  fn new(n: uint) -> UF {
    UF { vec: Vec::from_fn(n, |idx| idx) }
  }

  fn connected(&self, p: uint, q: uint) -> bool {
    self.vec[p] == self.vec[q]
  }

  fn union(&mut self, p: uint, q: uint) -> () {
    let pid = self.vec[p];
    let qid = self.vec[q];

    for i in range(0u, self.vec.len()) {
      if self.vec[i] == pid {
        *self.vec.get_mut(i as uint) = qid
      }
    }
  }
}

fn main() {
  let mut uf = UF::new(6);
  uf.union(4, 5);

  assert_eq!(true, uf.connected(4, 5))
  assert_eq!(true, uf.connected(1, 1))
  assert_eq!(false, uf.connected(2, 5))
}
