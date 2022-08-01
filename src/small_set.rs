use crate::{private::Seal, InnerVEBTree, VEBTree};
use core::ops::{BitAnd, BitOr, Not, Shl, Shr, Sub};

pub trait Bits:
    Copy
    + PartialEq
    + Eq
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + Not<Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
    + Sub<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
    fn leading_zeros(self) -> usize;
    fn trailing_zeros(self) -> usize;
}

macro_rules! impl_bits {
    ($type:ty) => {
        impl Bits for $type {
            fn zero() -> Self {
                0
            }
            fn one() -> Self {
                1
            }
            fn leading_zeros(self) -> usize {
                self.leading_zeros() as usize
            }
            fn trailing_zeros(self) -> usize {
                self.trailing_zeros() as usize
            }
        }
    };
}

impl_bits!(u16);
impl_bits!(u32);
impl_bits!(u64);
impl_bits!(u128);

/// Base case implementation of `VEBTree` for small integers.
/// Maintains a set of integers from
/// 0 to (exclusive) `1 << BITS = size_of::<T>()*8`.
/// using `T` as a collection of flags.
#[derive(Clone, Copy)]
pub struct SmallSet<const BITS: usize, T: Bits> {
    bits: T,
}

impl<const BITS: usize, T: Bits> Seal for SmallSet<BITS, T> {}

impl<const BITS: usize, T: Bits> SmallSet<BITS, T> {
    pub fn new() -> Self {
        Self { bits: T::zero() }
    }
}

impl<const BITS: usize, T: Bits> Default for SmallSet<BITS, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const BITS: usize, T: Bits> core::fmt::Debug for SmallSet<BITS, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<const BITS: usize, T: Bits> SmallSet<BITS, T> {
    pub fn capacity() -> usize {
        Self::CAPACITY
    }

    pub fn clear(&mut self) {
        self.bits = T::zero();
    }

    pub fn is_empty(&self) -> bool {
        self.bits == T::zero()
    }

    pub fn contains(&self, x: usize) -> bool {
        debug_assert!(x < Self::CAPACITY);
        self.bits >> x & T::one() != T::zero()
    }

    pub fn insert(&mut self, x: usize) -> bool {
        let was = self.contains(x);
        self.bits = self.bits | T::one() << x;
        !was
    }

    pub fn remove(&mut self, x: usize) -> bool {
        let was = self.contains(x);
        self.bits = self.bits & !(T::one() << x);
        was
    }

    pub fn next(&self, x: usize) -> Option<usize> {
        debug_assert!(x < Self::CAPACITY);
        let big_enough = self.bits & !((T::one() << x) - T::one());
        (big_enough != T::zero()).then(|| big_enough.trailing_zeros())
    }

    pub fn prev(&self, x: usize) -> Option<usize> {
        debug_assert!(x < Self::CAPACITY);
        let small_enough = if x == Self::CAPACITY - 1 {
            self.bits
        } else {
            self.bits & ((T::one() << (x + 1)) - T::one())
        };
        (small_enough != T::zero()).then(|| Self::CAPACITY - 1 - small_enough.leading_zeros())
    }

    pub fn first(&self) -> Option<usize> {
        (self.bits != T::zero()).then(|| self.bits.trailing_zeros())
    }

    pub fn last(&self) -> Option<usize> {
        (self.bits != T::zero()).then(|| Self::CAPACITY - 1 - self.bits.leading_zeros())
    }
}

impl<const BITS: usize, T: Bits> InnerVEBTree for SmallSet<BITS, T> {
    const BITS: usize = BITS;
}

impl<const BITS: usize, T: Bits> VEBTree for SmallSet<BITS, T> {
    fn capacity(&self) -> usize {
        Self::capacity()
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn contains(&self, x: usize) -> bool {
        self.contains(x)
    }

    fn insert(&mut self, x: usize) -> bool {
        self.insert(x)
    }

    fn remove(&mut self, x: usize) -> bool {
        self.remove(x)
    }

    fn next(&self, x: usize) -> Option<usize> {
        self.next(x)
    }

    fn prev(&self, x: usize) -> Option<usize> {
        self.prev(x)
    }

    fn first(&self) -> Option<usize> {
        self.first()
    }

    fn last(&self) -> Option<usize> {
        self.last()
    }
}
