# flat-veb

Fast implementation of van Emde Boas trees without internal allocation.

van Emde Boas tree is a data structure for maintaining
a set of integers of bounded size supporting the following queries:

* insert(x)   - inserts the integer x into the set
* remove(x)   - removes the integer x from the set
* contains(x) - returns whether the set contains x
* next(x)     - returns the smallest integer in the
                set that is greater or equal to x
* prev(x)     - returns the smallest integer in the
                set that is greater or equal to x

All of these use O(log log U) time,
and the structure uses O(U) space,
where U is the biggest integer you can put in the set.


## Usage
`SizedVEBTree` is generic over the a constant `usize`,
which is the number of bits in the integers it holds.
In other words, with `SizedVEBTree<X>`,
you can only insert elements with
value less than `1 << X`.
```rust
let mut tree = flat_veb::SizedVEBTree::<24>::new();

// note that SizedVEBTree<24> is a quite big object,
// using over 2 MB while empty, but the size
// doesn't increase when elements are inserted.

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

To get a `VEBTree` with run-time decided capacity:
```rust
let mut tree = flat_veb::new_with_capacity(100);

// The capacity becomes the next power of two
assert_eq!(tree.capacity(), 128);
assert_eq!(tree.capacity(), flat_veb::new_with_capacity(128).capacity());
assert_ne!(tree.capacity(), flat_veb::new_with_capacity(129).capacity());

assert_eq!(tree.insert(127), true);
//tree.insert(128); // panics
```


## Performance

It is natural to use internal heap allocation and indirection to implement
recursive data structures like van Emde Boas tree, but this implementation
avoid that to be faster, at the cost of a bit cumbersome API.

A `BTreeSet` can do all of the operations a `VEBTree` can and much more,
but is slower.
A `VEBTree` is more appropriate if there are enough operations that
the speed improvement matters, but the integers are small enough that
the `VEBTree` doesn't take too much space.
If there are many entries compared to how big they can be,
`VEBTree` can even use less memory than a `BTreeSet` of integers.

`VEBTree` is about 10 times faster than `BTreeSet` on tests
downloaded from <https://judge.yosupo.jp/problem/predecessor_problem>,
but this includes IO, which is probably a significant
amount of the time spent for the `VEBTree`. Better benchmarks are needed.


## Todo

- better benchmarks
- create a function to return a Box<dyn VEBTree> of appropriate capacity
- reverse iterator

License: MIT
