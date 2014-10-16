# Copyright 2014 Jose Narvaez. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://ruby-lang.org/COPYRIGHT.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

#
#
# Implementation of "Quick Union" algorithm for the "Dynamic Conectivity"
# problem of lecture 1 with the weight optimisation in order to avoid the
# tall trees. That is: We add another array meant to keep track of the
# size of each tree and we always link the root the smaller tree to the
# root of the taller tree.
#
#

class UF
  attr_accessor :vec, :sizes

  def initialize(n)
    @vec = 0.upto(n).to_a
    @sizes = 0.upto(n).to_a
  end

  def root(i)
    while i != @vec[i] do
      i = @vec[i]
    end

    i
  end

  def connected?(p, q)
    root(p) == root(q)
  end

  def union(p, q)
    i = root(p)
    j = root(q)

    return if i == j

    if @sizes[i] < @sizes[j]
      @vec[i] = j
      @sizes[j] += @sizes[i]
    else
      @vec[j] = i
      @sizes[i] += @sizes[j]
    end
  end
end

def main
  uf = UF.new(10)

  uf.union(4,1)
  uf.union(6,8)
  uf.union(0,9)
  uf.union(0,7)
  uf.union(5,7)
  uf.union(3,5)
  uf.union(4,6)
  uf.union(1,0)
  uf.union(9,2)
end

main()
