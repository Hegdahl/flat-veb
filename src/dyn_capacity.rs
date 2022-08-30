extern crate alloc;

use crate::{InnerVEBTree, SizedVEBTree, VEBTree};
use alloc::boxed::Box;
use deep_maybe_uninit::IsDeepMaybeUninit;

/// Gets a new empty boxed instance of `T`
/// initialized without storing
/// the object on the stack.
///
/// This is necessary because `VEBTree`s
/// with high capacities are very big objects
/// using more space than a typical stack
/// size limit, and from experimentation
/// the `T::default()` call inside `Box::default()`
/// doesn't get inlined, leading to storing `T`
/// on the stack.
pub fn new_boxed<T: InnerVEBTree>() -> Box<T> {
    let mut b = T::boxed_uninit();
    T::init(&mut b);
    unsafe { b.boxed_assume_init() }
}

/// Get the smallest capacity `VEBTree` implementation which
/// can hold integers of size at least `capacity - 1`.
///
/// # Panics
///
/// The function panics if given an absurdly high capacity,
/// because there is no type to return an instance of with that capacity.
///
/// But it probably fails for other reasons for smaller
/// capacities than that, like running out of memory.
#[must_use]
pub fn new_with_capacity(capacity: usize) -> Box<dyn VEBTree> {
    macro_rules! inner {
        ($n:expr, T T T T $($tail:tt)*) => {
            if capacity <= SizedVEBTree::<{ $n }>::CAPACITY {
                return new_boxed::<SizedVEBTree<{ $n }>>();
            }

            inner! {($n+1), T T T $($tail)*}
        };
        ($n:expr, T T T) => {}
    }

    inner! {4,
        T T T T T T T T
        T T T T T T T T
        T T T T T T T T
        T T T T T T T T
        T T T T T T T T
        T T T T T T T T
        T
    }

    panic!("Too high capacity: {capacity}.");
}

/// Get the smallest capacity `VEBTree` implementation which
/// can hold integers with at least `bits` bits.
///
/// # Panics
///
/// Panics if `1 << bits` is not representable in a `usize`,
/// and if `new_with_capacity(1 << bits)` panics.
#[must_use]
pub fn new_with_bits(bits: usize) -> Box<dyn VEBTree> {
    assert!(
        bits < core::mem::size_of::<usize>() * 8,
        "Too high number of bits: {bits}.
        Can not represent a size that big on this platform."
    );
    new_with_capacity(1 << bits)
}
