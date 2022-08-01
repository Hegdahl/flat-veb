extern crate alloc;
use crate::{outer, small_set::SmallSet, InnerVEBTree};

/// Trait used as a function taking the integer `BITS`
/// as an argument, returning a `VEBTree` holding integers
/// of that many bits.
/// This is just an implementation detail for `SizedVEBTree`
pub trait GetVEBTreeSize<const BITS: usize> {
    /// Type of `VEBTree` holding integers with `BITS` bits.
    type Type: InnerVEBTree;
}

impl GetVEBTreeSize<4> for () {
    type Type = SmallSet<4, u16>;
}
impl GetVEBTreeSize<5> for () {
    type Type = SmallSet<5, u32>;
}
impl GetVEBTreeSize<6> for () {
    type Type = SmallSet<6, u64>;
}
impl GetVEBTreeSize<7> for () {
    type Type = SmallSet<7, u128>;
}

macro_rules! make_veb_tree_sizes {
    ($n:expr, T T T T T T T T $($tail:tt)*) => {
        impl GetVEBTreeSize<{ $n }> for () {
            type Type = outer::VEBTree<
                <() as GetVEBTreeSize<{ $n / 2 }>>::Type,
                <() as GetVEBTreeSize<{ ($n + 1) / 2 }>>::Type,
            >;
        }
        make_veb_tree_sizes! {($n+1), T T T T T T T $($tail)*}
    };
    ($n:expr, T T T T T T T) => {}
}

make_veb_tree_sizes! {8,
    T T T T T T T T
    T T T T T T T T
    T T T T T T T T
    T T T T T T T T
    T T T T T T T T
    T T T T T T T T
    T T T T T T T T
    T
}

/// `VEBTree` which can hold integers with BITS bits in them.
/// In other words, the entries have to be smaller than `1 << BITS`.
pub type SizedVEBTree<const BITS: usize> = <() as GetVEBTreeSize<BITS>>::Type;
