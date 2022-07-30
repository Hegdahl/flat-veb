//! Implementation of van Emde Boas Tree
//! of constant size, without internal allocation.
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
