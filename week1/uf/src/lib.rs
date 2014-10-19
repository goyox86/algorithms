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

Implementation of "Quick Union" algorithm for the "Dynamic Conectivity"
problem of lecture 1 with the weight optimisation in order to avoid the
tall trees. That is: We add another array meant to keep track of the
size of each tree and we always link the root the smaller tree to the
root of the taller tree.

*/

#![feature(slicing_syntax)]

pub struct UF {
  vec: Vec<uint>,
  sizes: Vec<uint>
}

impl UF {
  /// Constructs a new `UF` of size `n`.
  ///
  ///
  ///
  /// # Example
  ///
  /// ```
  /// let mut uf = UF::new(10);
  /// ```
  pub fn new(n: uint) -> UF {
    UF {
      vec: Vec::from_fn(n, |idx| idx),
      sizes: Vec::from_fn(n, |idx| 1)
    }
  }

  /// Finds the root of `i`.
  ///
  /// Iterates over `vec`, and returns the root of `i`
  ///
  /// # Example
  ///
  /// ```
  /// let mut uf = UF::new(10);
  /// uf.union(4, 5);
  /// assert_eq!(5, uf.root(4));
  /// ```
  fn root(&self, i: uint) -> uint {
    let mut j = i;

    while j != self.vec[j] {
      j = self.vec[j];
    }

    j
  }

  /// Determines if `p` component is connected to `q` component.
  ///
  /// # Example
  ///
  /// ```
  /// let mut uf = UF::new(10);
  /// uf.union(4, 5);
  /// assert_eq!(true, uf.connected(4, 5));
  /// ```
  pub fn connected(&self, p: uint, q: uint) -> bool {
    self.root(p) == self.root(q)
  }

  /// Connects `p` component to `q` component.
  ///
  /// # Example
  ///
  /// ```
  /// let mut uf = UF::new(10);
  /// uf.union(4, 5);
  /// assert_eq!(true, uf.connected(4, 5));
  /// ```
  pub fn union(&mut self, p: uint, q: uint) -> () {
    let i = self.root(p);
    let j = self.root(q);

    if i == j { return };

    if self.sizes[i] < self.sizes[j] {
      self.vec[mut][i] = j;
      self.sizes[mut][j] += self.sizes[i];
    } else {
      self.vec[mut][j] = i;
      self.sizes[mut][i] += self.sizes[j];
    }
  }

  /// Shows `vec`.
  ///
  /// # Example
  ///
  /// ```
  /// let mut uf = UF::new(10);
  /// uf.union(4, 5);
  /// println!("{}", uf.show_vec());
  /// ```
  pub fn show_vec(&self) -> () {
    println!("{}", self.vec)
  }

  /// Shows `sizes`.
  ///
  /// # Example
  ///
  /// ```
  /// let mut uf = UF::new(10);
  /// uf.union(4, 5);
  /// println!("{}", uf.show_sizes());
  /// ```
  pub fn show_sizes(&self) -> () {
    println!("{}", self.sizes)
  }
}
