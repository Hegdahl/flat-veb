//! Fast implementation of vEB trees without internal allocation.
//!
//! van Emde Boas tree is a data structure for maintaining
//! a set of integers of bounded size supporting the following queries:
//!   insert(x) - inserts the integer x into the set
//!   remove(x) - removes the integer x from the set
//! contains(x) - returns whether the set contains x
//!     next(x) - returns the smallest integer in the
//!               set that is greater or equal to x
//!     prev(x) - returns the smallest integer in the
//!               set that is greater or equal to x
//!
//! All of these use O(log log U) time,
//! and the structure uses O(U) space,
//! where U is the biggest integer you can put in the set.
//! 
//!
//! # Usage
//! use the trait `VEBTree` and the type `VEBTreeX`
//! where X is the number of bits in the elements you will insert.
//! In other words, with `VEBTreeX` you can only insert elements with
//! value less than 1 << X.
//! ```
//! use flat_veb::{VEBTree, VEBTree24};
//! let mut tree = VEBTree24::new();
//!
//! // note that VEBTree24 is a quite big object, using over 2 MB while empty,
//! // but the size doesn't increase when elements are inserted.
//!
//! assert_eq!(tree.insert(123), true); // returns true if it wasn't already there
//! assert_eq!(tree.insert(1337), true);
//! assert_eq!(tree.insert(123), false); // false because it was already there
//!
//! assert_eq!(tree.contains(123), true);
//! assert_eq!(tree.contains(42), false);
//!
//! assert_eq!(tree.next(42), Some(123));
//! assert_eq!(tree.next(123), Some(123));
//! assert_eq!(tree.next(124), Some(1337));
//!
//! assert_eq!(tree.remove(1337), true);
//! assert_eq!(tree.remove(1337), false); // it's not there when removing it the second time
//!
//! assert_eq!(tree.next(124), None); // there is no element in te set >= 124
//! ```
//!
//!
//! # Performance
//! 
//! It is natural to use internal heap allocation and indirection to implement
//! recursive data structures like vEB tree, but this implementation
//! avoid that to be faster, at the cost of a bit cumbersome API.
//!
//! A BTreeSet can do all of the operations a vEB tree can and much more,
//! but is slower.
//! A vEB tree is more appropriate if there are enough operations that
//! the speed improvement matters, but the integers are small enough that
//! the vEB tree doesn't take too much space.
//!
//! vEB tree is about 10 times faster than BTreeSet on tests
//! downloaded from <https://judge.yosupo.jp/problem/predecessor_problem>,
//! but this includes IO, which is probably a significant
//! amount of the time spent for the vEB tree. Better benchmarks are needed.
//! 
//! 
//! # Todo
//! 
//! - better benchmarks
//! - create a function to return a Box<dyn VEBTree> of appropriate capacity
//! - reverse iterator
#![no_std]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![warn(missing_docs, missing_debug_implementations)]

#[allow(missing_docs)]
mod aliases;
mod outer;
mod small_set;
pub use aliases::*;

/// Common trait for the different implementations
/// of sets for different sizes.
pub trait VEBTree: Copy + Sized + Default + core::fmt::Debug {
    /// The set can hold values with BITS bits.
    const BITS: usize;

    /// The set can hold values in [0, CAPACITY)
    const CAPACITY: usize = 1 << Self::BITS;

    /// Mask for which part of usize is
    /// small enough to be held in this set.
    const MASK: usize = Self::CAPACITY - 1;

    /// Makes a new, empty vEB-tree-like object.
    fn new() -> Self {
        Default::default()
    }

    /// Clears the set, removing all elements.
    fn clear(&mut self);

    /// Returns true if the set contains no elements.
    fn is_empty(&self) -> bool;

    /// Returns true if the set contains x.
    fn contains(&self, x: usize) -> bool;

    /// Adds x to the set.
    ///
    /// If the set did not have x present, true is returned.
    ///
    /// If the set did have x present, false is returned,
    /// and the entry is not updated.
    fn insert(&mut self, x: usize) -> bool;

    /// If the set contains x,
    /// removes it from the set.
    /// Returns whether such an element was present.
    fn remove(&mut self, x: usize) -> bool;

    /// Returns the first element in the set that is
    /// greater or equal to x, if any.
    fn next(&self, x: usize) -> Option<usize>;

    /// Returns the last element in the set that is
    /// smaller or equal to x, if any.
    fn prev(&self, x: usize) -> Option<usize>;

    /// Returns the first element in the set, if any.
    /// This element is always the minimum of all elements in the set.
    fn first(&self) -> Option<usize>;

    /// Returns the last element in the set, if any.
    /// This element is always the maximum of all elements in the set.
    fn last(&self) -> Option<usize>;

    /// Returns an iterator over the values in the set.
    fn iter(&self) -> VEBIterator<'_, Self> {
        VEBIterator {
            tree: self,
            next_start: 0,
        }
    }
}

/// This struct is created by the iter method
/// on objects implementing VEBOperations
#[derive(Debug)]
pub struct VEBIterator<'a, T: VEBTree> {
    tree: &'a T,
    next_start: usize,
}

impl<'a, T: VEBTree> Iterator for VEBIterator<'a, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_start == T::CAPACITY {
            None
        } else {
            let value = self.tree.next(self.next_start)?;
            self.next_start = value + 1;
            Some(value)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::VEBTree;

    macro_rules! make_tests {
        ($name:ident, $type:ty) => {
            mod $name {
                use crate::VEBTree;

                type T = $type;

                #[test]
                fn empty_works() {
                    let mut s = T::new();
                    assert!(s.is_empty());
                    s.clear();
                    assert!(s.is_empty());

                    for x in 0..T::CAPACITY.min(1000) {
                        assert!(!s.contains(x));
                    }
                }

                #[test]
                fn small_collect() {
                    let mut s = T::new();
                    s.insert(2);
                    s.insert(4);
                    s.insert(6);

                    let mut it = s.iter();
                    assert_eq!(it.next(), Some(2));
                    assert_eq!(it.next(), Some(4));
                    assert_eq!(it.next(), Some(6));
                    assert_eq!(it.next(), None);
                }

                #[test]
                fn spaced_collect() {
                    let spacing = (T::CAPACITY / 20).max(2);
                    let mut s = T::new();

                    for x in (0..T::CAPACITY).step_by(spacing) {
                        s.insert(x);
                    }

                    let mut iter = s.iter();

                    for x in (0..T::CAPACITY).step_by(spacing) {
                        assert_eq!(iter.next(), Some(x));
                    }
                    assert_eq!(iter.next(), None);
                }
            }
        };
    }

    make_tests! {size4, crate::VEBTree4}
    make_tests! {size5, crate::VEBTree5}
    make_tests! {size6, crate::VEBTree6}
    make_tests! {size7, crate::VEBTree7}
    make_tests! {size8, crate::VEBTree8}
    make_tests! {size9, crate::VEBTree9}
    make_tests! {size10, crate::VEBTree10}
    make_tests! {size11, crate::VEBTree11}
    make_tests! {size12, crate::VEBTree12}

    #[test]
    fn correct_bits() {
        assert_eq!(crate::VEBTree4::BITS, 4);
        assert_eq!(crate::VEBTree5::BITS, 5);
        assert_eq!(crate::VEBTree6::BITS, 6);
        assert_eq!(crate::VEBTree7::BITS, 7);
        assert_eq!(crate::VEBTree8::BITS, 8);
        assert_eq!(crate::VEBTree9::BITS, 9);
        assert_eq!(crate::VEBTree10::BITS, 10);
        assert_eq!(crate::VEBTree11::BITS, 11);
        assert_eq!(crate::VEBTree12::BITS, 12);
        assert_eq!(crate::VEBTree13::BITS, 13);
        assert_eq!(crate::VEBTree14::BITS, 14);
        assert_eq!(crate::VEBTree15::BITS, 15);
        assert_eq!(crate::VEBTree16::BITS, 16);
        assert_eq!(crate::VEBTree17::BITS, 17);
        assert_eq!(crate::VEBTree18::BITS, 18);
        assert_eq!(crate::VEBTree19::BITS, 19);
        assert_eq!(crate::VEBTree20::BITS, 20);
        assert_eq!(crate::VEBTree21::BITS, 21);
        assert_eq!(crate::VEBTree22::BITS, 22);
        assert_eq!(crate::VEBTree23::BITS, 23);
        assert_eq!(crate::VEBTree24::BITS, 24);
        assert_eq!(crate::VEBTree25::BITS, 25);
        assert_eq!(crate::VEBTree26::BITS, 26);
        assert_eq!(crate::VEBTree27::BITS, 27);
        assert_eq!(crate::VEBTree28::BITS, 28);
        assert_eq!(crate::VEBTree29::BITS, 29);
        assert_eq!(crate::VEBTree30::BITS, 30);
        assert_eq!(crate::VEBTree31::BITS, 31);
        assert_eq!(crate::VEBTree32::BITS, 32);
    }
}
