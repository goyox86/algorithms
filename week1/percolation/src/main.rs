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

Write a program to estimate the value of the percolation threshold via Monte Carlo
simulation.

*/

#![feature(slicing_syntax)]

extern crate uf;

use std::rand;
use std::rand::Rng;
use uf::UF;

struct Percolation {
  n: uint,
  opened_count: uint,
  uf: UF
}

impl Percolation {
  fn new(n: uint) -> Percolation {
    let mut obj = Percolation {
      n: n,
      opened_count: 0u,
      uf: UF::new(n * n)
    };

    //building the 'top' virtual site.
    let top_left = obj.index_of(1, 1);
    for i in range(1u, obj.n + 1) {
      let q = obj.index_of(i, 1);
      obj.uf.union(top_left, q)
    }

    //building the 'bottom' virtual site.
    let bottom_left = obj.index_of(1, obj.n);
    for i in range(1u, obj.n + 1) {
      let q = obj.index_of(i, obj.n);
      obj.uf.union(bottom_left, q)
    }

    obj
  }

  fn open(&mut self, i: uint, j: uint) {
    if self.is_open(i, j) { return; }

    let me = self.index_of(i, j);
    if j > 1 {
      let left = self.index_of(i, j - 1);
      self.uf.union(me, left);
    }

    if j < self.n {
      let right = self.index_of(i, j + 1);
      self.uf.union(me, right);
    }

    if i > 1 {
      let up = self.index_of(i - 1, j);
      self.uf.union(me, up);
    }

    if i > self.n {
      let down = self.index_of(i + 1, j);
      self.uf.union(me, down);
    }

    self.opened_count += 1;
  }

  fn is_open(&self, i: uint, j: uint) -> bool {
    let me = self.index_of(i, j);
    let mut res = false;

    if j > 1 {
      let left = self.index_of(i, j - 1);
      if self.uf.connected(me, left) {
        res = true;
      }
    }

    if j < self.n {
      let right = self.index_of(i, j + 1);
      if self.uf.connected(me, right) {
        res = true;
      }
    }

    if i > 1 {
      let up = self.index_of(i - 1, j);
      if self.uf.connected(me, up) {
        res = true;
      }
    }

    if i > self.n {
      let down = self.index_of(i + 1, j);
      if self.uf.connected(me, down) {
        res = true
      }
    }

    res
  }

  fn is_full(&mut self, i: uint, j: uint) -> bool {
    let top_left = self.index_of(1, 1);
    self.uf.connected(self.index_of(i, j), top_left)
  }

  fn percolates(&mut self) -> bool {
    let n = self.n;
    for i in range(1u, n + 1) {
      let is_full = self.is_full(i, n);
      if is_full  { return true }
    }

    false
  }

  fn index_of(&self, i: uint, j: uint) -> uint {
    ((i - 1) * self.n) + (j - 1)
  }
}

struct PercolationStats {
  n: uint,
  t: uint,
  tresholds: Vec<f64>
}

impl PercolationStats {
  fn new(n: uint, t: uint) -> PercolationStats {
    let mut obj = PercolationStats {
      n: n,
      t: t,
      tresholds: Vec::with_capacity(t)
    };

    obj
  }

  fn run(&mut self) {
    for x in range(0i, self.t as int) {
      let mut p = Percolation::new(self.n);
      let mut rng = rand::task_rng();

      loop {
        let mut i = rng.gen::<i16>();
        while (i > self.n as i16) || i <= 0 || (i > self.n as i16) {
          i = rng.gen::<i16>()
        }
        let mut j = rng.gen::<i16>();
        while (j > self.n as i16) || j <= 0 || (j > self.n as i16) {
          j = rng.gen::<i16>()
        }
        p.open(i as uint, j as uint);
        if p.percolates() {
          self.tresholds.push(p.opened_count as f64 / (self.n * self.n) as f64);
          break;
        }
      }
    }
  }

  fn mean(&self) -> f64 {
    let mut total = 0.0f64;
    for i in range(0u, self.t) {
      total += self.tresholds[i] as f64;
    }

    total / (self.t as f64)
  }

  fn stddev(&self) -> f64 {
    let mut total = 0.0f64;
    let mean = self.mean();
    for i in range(0u, self.t) {
      total += (self.tresholds[i] - mean) * (self.tresholds[i] - mean);
    }

    total / ((self.t - 1) as f64)
  }

  fn confidence_lo(&self) -> f64 {
    let mean = self.mean();
    let stddev = self.stddev();

    mean - (1.96 * stddev / (self.t as f64).sqrt())
  }

  fn confidence_hi(&self) -> f64 {
    let mean = self.mean();
    let stddev = self.stddev();

    mean + (1.96 * stddev / (self.t as f64).sqrt())
  }
}

fn main() {
  let n = 30;
  let t = 1000;

  let mut ps = PercolationStats::new(n, t);
  ps.run();
  println!("mean: {}", ps.mean());
  println!("stddev: {}", ps.stddev());
  println!("95% confidence interval {} {}", ps.confidence_lo(),
                                            ps.confidence_hi());
}
