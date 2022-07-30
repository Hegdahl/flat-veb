use super::VEBTree;
use core::ops::{BitAnd, BitOr, Not, Shl, Shr, Sub};

trait Bits:
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

/// Do vEB tree operations
/// on integers in [0, 64)
#[derive(Clone, Copy)]
pub struct SmallSet<const BITS: usize, T: Copy> {
    bits: T,
}

impl<const BITS: usize, T: Bits> Default for SmallSet<BITS, T> {
    fn default() -> Self {
        Self { bits: T::zero() }
    }
}

impl<const BITS: usize, T: Bits> core::fmt::Debug for SmallSet<BITS, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<const BITS: usize, T: Bits> VEBTree for SmallSet<BITS, T> {
    const BITS: usize = BITS;

    fn clear(&mut self) {
        self.bits = T::zero();
    }

    fn is_empty(&self) -> bool {
        self.bits == T::zero()
    }

    fn contains(&self, x: usize) -> bool {
        debug_assert!(x < Self::CAPACITY);
        self.bits >> x & T::one() != T::zero()
    }

    fn insert(&mut self, x: usize) -> bool {
        let was = self.contains(x);
        self.bits = self.bits | T::one() << x;
        !was
    }

    fn remove(&mut self, x: usize) -> bool {
        let was = self.contains(x);
        self.bits = self.bits & !(T::one() << x);
        was
    }

    fn next(&self, x: usize) -> Option<usize> {
        debug_assert!(x < Self::CAPACITY);
        let big_enough = self.bits & !((T::one() << x) - T::one());
        (big_enough != T::zero()).then(|| big_enough.trailing_zeros())
    }

    fn prev(&self, x: usize) -> Option<usize> {
        debug_assert!(x < Self::CAPACITY);
        let small_enough = if x == Self::CAPACITY - 1 {
            self.bits
        } else {
            self.bits & ((T::one() << x + 1) - T::one())
        };
        (small_enough != T::zero()).then(|| Self::CAPACITY - 1 - small_enough.leading_zeros())
    }

    fn first(&self) -> Option<usize> {
        (self.bits != T::zero()).then(|| self.bits.trailing_zeros())
    }

    fn last(&self) -> Option<usize> {
        (self.bits != T::zero()).then(|| Self::CAPACITY - 1 - self.bits.leading_zeros())
    }
}
