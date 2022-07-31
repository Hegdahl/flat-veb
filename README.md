# flat-veb

Fast implementation of vEB trees without internal allocation.

van Emde Boas tree is a data structure for maintaining
a set of integers of bounded size supporting the following queries:
  insert(x) - inserts the integer x into the set
  remove(x) - removes the integer x from the set
contains(x) - returns whether the set contains x
    next(x) - returns the smallest integer in the
              set that is greater or equal to x
    prev(x) - returns the smallest integer in the
              set that is greater or equal to x

All of these use $\mathcal{O}(\log \log U)$ time,
and the structure uses $\matchcal{O}(U)$ space,
where U is the biggest integer you can put in the set.


## Usage
use the trait `VEBTree` and the type `VEBTreeX`
where X is the number of bits in the elements you will insert.
In other words, with `VEBTreeX` you can only insert elements with
value less than 1 << X.
```rust
use flat_veb::{VEBTree, VEBTree24};
let mut tree = VEBTree24::new();

// note that VEBTree24 is a quite big object, using over 2 MB while empty,
// but the size doesn't increase when elements are inserted.

assert_eq!(tree.insert(123), true); // returns true if it wasn't already there
assert_eq!(tree.insert(1337), true);
assert_eq!(tree.insert(123), false); // false because it was already there

assert_eq!(tree.contains(123), true);
assert_eq!(tree.contains(42), false);

assert_eq!(tree.next(42), Some(123));
assert_eq!(tree.next(123), Some(123));
assert_eq!(tree.next(124), Some(1337));

assert_eq!(tree.remove(1337), true);
assert_eq!(tree.remove(1337), false); // it's not there when removing it the second time

assert_eq!(tree.next(124), None); // there is no element in te set >= 124
```


## Performance

It is natural to use internal heap allocation and indirection to implement
recursive data structures like vEB tree, but this implementation
avoid that to be faster, at the cost of a bit cumbersome API.

A BTreeSet can do all of the operations a vEB tree can and much more,
but is slower.
A vEB tree is more appropriate if there are enough operations that
the speed improvement matters, but the integers are small enough that
the vEB tree doesn't take too much space.

vEB tree is about 10 times faster than BTreeSet on tests
downloaded from <https://judge.yosupo.jp/problem/predecessor_problem>,
but this includes IO, which is probably a significant
amount of the time spent for the vEB tree. Better benchmarks are needed.


## Todo

- better benchmarks
- create a function to return a Box<dyn VEBTree> of appropriate capacity
- reverse iterator

License: MIT
