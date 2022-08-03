//! Fast implementation of van Emde Boas trees without internal allocation.
//!
//! van Emde Boas tree is a data structure for maintaining
//! a set of integers of bounded size supporting the following queries:
//!
//! * insert(x)   - inserts the integer x into the set
//! * remove(x)   - removes the integer x from the set
//! * contains(x) - returns whether the set contains x
//! * next(x)     - returns the smallest integer in the
//!                 set that is greater or equal to x
//! * prev(x)     - returns the smallest integer in the
//!                 set that is greater or equal to x
//!
//! All of these use O(log log U) time,
//! and the structure uses O(U) space,
//! where U is the biggest integer you can put in the set.
//!
//!
//! # Usage
//! `SizedVEBTree` is generic over the a constant `usize`,
//! which is the number of bits in the integers it holds.
//! In other words, with `SizedVEBTree<X>`,
//! you can only insert elements with
//! value less than `1 << X`.
//! ```
//! let mut tree = flat_veb::SizedVEBTree::<24>::new();
//!
//! // note that SizedVEBTree<24> is a quite big object,
//! // using over 2 MB while empty, but the size
//! // doesn't increase when elements are inserted.
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
//! To get a `VEBTree` with run-time decided capacity:
//! ```
//! let mut tree = flat_veb::new_with_capacity(100);
//!
//! // The capacity becomes the next power of two
//! assert_eq!(tree.capacity(), 128);
//! assert_eq!(tree.capacity(), flat_veb::new_with_capacity(128).capacity());
//! assert_ne!(tree.capacity(), flat_veb::new_with_capacity(129).capacity());
//!
//! assert_eq!(tree.insert(127), true);
//! //tree.insert(128); // panics
//! ```
//!
//!
//! # Performance
//!
//! It is natural to use internal heap allocation and indirection to implement
//! recursive data structures like van Emde Boas tree, but this implementation
//! avoid that to be faster, at the cost of a bit cumbersome API.
//!
//! A `BTreeSet` can do all of the operations a `VEBTree` can and much more,
//! but is slower.
//! A `VEBTree` is more appropriate if there are enough operations that
//! the speed improvement matters, but the integers are small enough that
//! the `VEBTree` doesn't take too much space.
//! If there are many entries compared to how big they can be,
//! `VEBTree` can even use less memory than a `BTreeSet` of integers.
//!
//! `VEBTree` is about 10 times faster than `BTreeSet` on tests
//! downloaded from <https://judge.yosupo.jp/problem/predecessor_problem>,
//! but this includes IO, which is probably a significant
//! amount of the time spent for the `VEBTree`. Better benchmarks are needed.
//!
//!
//! # Todo
//!
//! - better benchmarks
//! - reverse iterator
#![no_std]
#![cfg(feature = "dyn_capacity")]
#![warn(missing_docs, missing_debug_implementations)]
#![warn(clippy::pedantic)]

mod outer;
mod sizes;
mod small_set;
pub use sizes::SizedVEBTree;

#[cfg(feature = "dyn_capacity")]
mod dyn_capacity;
#[cfg(feature = "dyn_capacity")]
pub use dyn_capacity::{new_with_bits, new_with_capacity};

mod private {
    /// Both a promise that the type is zeroable,
    /// and functions as a seal for the crate,
    /// meaning traits implying `ZeroableSeal`
    /// can not be implemented downstream.
    /// 
    /// 
    /// # Safety
    /// 
    /// Should only be implemented by types which
    /// are in a valid state when the underlying
    /// memory is set to 0.
    pub unsafe trait ZeroableSeal {}
}

/// Constants and implied traits for the `VEBTree` trait,
/// separated out to make `VEBTree` object safe.
pub trait InnerVEBTree: Copy + Sized + Default + VEBTree {
    /// The set can hold values with BITS bits.
    const BITS: usize;

    /// The set can hold values in [0, CAPACITY)
    const CAPACITY: usize = 1 << Self::BITS;
}

/// Fast implementation of van Emde Boas trees without internal allocation.
/// This is a trait instead of a struct to generalize over
/// the different types used for different capacities.
///
/// To take an a reference to a `VEBTree` of any capacity as an argument,
/// use `&impl VEBTree` in the signature.
///
/// The type of a specific size is `SizedVEBTree<BITS>`.
pub trait VEBTree: private::ZeroableSeal + core::fmt::Debug {
    /// Trait object version of `VEBTreeWithConstants::CAPACITY`.
    fn capacity(&self) -> usize;

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
    fn iter(&self) -> VEBIterator<'_>
    where
        Self: Sized,
    {
        VEBIterator {
            tree: self,
            next_start: 0,
        }
    }
}

/// This struct is created by the iter method
/// on objects implementing `VEBTree`.
#[derive(Debug)]
pub struct VEBIterator<'a> {
    tree: &'a dyn VEBTree,
    next_start: usize,
}

impl<'a> Iterator for VEBIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_start == self.tree.capacity() {
            None
        } else {
            let value = self.tree.next(self.next_start)?;
            self.next_start = value + 1;
            Some(value)
        }
    }
}
