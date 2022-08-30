use crate::{
    private::{ConditionalHasDeepMaybeUninit, Sealed},
    InnerVEBTree,
};
#[cfg(feature = "dyn_capacity")]
use deep_maybe_uninit::{DeepMaybeUninit, HasDeepMaybeUninit};

/// Recursive implementation of a van Emde Boas Tree.
#[cfg_attr(feature = "dyn_capacity", derive(DeepMaybeUninit))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VEBTree<const UPPER_CAPACITY: usize, Upper: InnerVEBTree, Lower: InnerVEBTree>
where
    [(); UPPER_CAPACITY]: ConditionalHasDeepMaybeUninit,
{
    upper: Upper,
    lower: [Lower; UPPER_CAPACITY],
    min: usize,
    max: usize,
}

impl<const UPPER_CAPACITY: usize, Upper: InnerVEBTree, Lower: InnerVEBTree> Sealed
    for VEBTree<UPPER_CAPACITY, Upper, Lower>
where
    [(); UPPER_CAPACITY]: ConditionalHasDeepMaybeUninit,
{
}

impl<const UPPER_CAPACITY: usize, Upper: InnerVEBTree, Lower: InnerVEBTree> Default
    for VEBTree<UPPER_CAPACITY, Upper, Lower>
where
    [(); UPPER_CAPACITY]: ConditionalHasDeepMaybeUninit,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const UPPER_CAPACITY: usize, Upper: InnerVEBTree, Lower: InnerVEBTree> core::fmt::Debug
    for VEBTree<UPPER_CAPACITY, Upper, Lower>
where
    [(); UPPER_CAPACITY]: ConditionalHasDeepMaybeUninit,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_set().entries(crate::VEBTree::iter(self)).finish()
    }
}

impl<const UPPER_CAPACITY: usize, Upper: InnerVEBTree, Lower: InnerVEBTree> InnerVEBTree
    for VEBTree<UPPER_CAPACITY, Upper, Lower>
where
    [(); UPPER_CAPACITY]: ConditionalHasDeepMaybeUninit,
{
    const BITS: usize = Upper::BITS + Lower::BITS;
}

impl<const UPPER_CAPACITY: usize, Upper: InnerVEBTree, Lower: InnerVEBTree>
    VEBTree<UPPER_CAPACITY, Upper, Lower>
where
    [(); UPPER_CAPACITY]: ConditionalHasDeepMaybeUninit,
{
    pub fn new() -> Self {
        Self {
            upper: Default::default(),
            lower: [Default::default(); UPPER_CAPACITY],
            min: usize::MAX,
            max: usize::MAX,
        }
    }

    #[cfg(feature = "dyn_capacity")]
    pub fn init(value: &mut <Self as HasDeepMaybeUninit>::AsDeepMaybeUninit) {
        Upper::init(&mut value.upper);
        for lower in value.lower.iter_mut() {
            Lower::init(lower);
        }
        value.min = usize::MAX.forget_init();
        value.max = usize::MAX.forget_init();
    }

    fn ul(x: usize) -> (usize, usize) {
        let ux = x >> Lower::BITS;
        let lx = x & (Lower::CAPACITY - 1);
        (ux, lx)
    }

    pub fn capacity() -> usize {
        Self::CAPACITY
    }

    pub fn clear(&mut self) {
        self.upper.clear();
        for low in &mut self.lower {
            low.clear();
        }
        self.min = usize::MAX;
        self.max = usize::MAX;
    }

    pub fn is_empty(&self) -> bool {
        self.min == usize::MAX
    }

    pub fn contains(&self, x: usize) -> bool {
        debug_assert!(x < Self::CAPACITY);

        if x < self.min {
            return false;
        }

        if x > self.max {
            return false;
        }

        if x == self.min {
            return true;
        }

        if x == self.max {
            return true;
        }

        let (ux, lx) = Self::ul(x);
        self.lower[ux].contains(lx)
    }

    pub fn insert(&mut self, mut x: usize) -> bool {
        debug_assert!(x < Self::CAPACITY);

        if self.is_empty() {
            self.min = x;
            self.max = x;
            return true;
        }

        if x < self.min {
            core::mem::swap(&mut x, &mut self.min);
        }

        if x == self.min {
            return false;
        }

        if x > self.max {
            self.max = x;
        }

        let (ux, lx) = Self::ul(x);
        if self.lower[ux].is_empty() {
            self.upper.insert(ux);
        }
        self.lower[ux].insert(lx)
    }

    pub fn remove(&mut self, mut x: usize) -> bool {
        debug_assert!(x < Self::CAPACITY);

        if self.min == self.max {
            return if x == self.min {
                self.min = usize::MAX;
                self.max = 0;
                true
            } else {
                false
            };
        }

        if x == self.min {
            x = self.next(x + 1).expect("self.min != self.max");
            self.min = x;
        }

        let (ux, lx) = Self::ul(x);
        if self.lower[ux].remove(lx) {
            if self.lower[ux].is_empty() {
                self.upper.remove(ux);
            }

            if x != self.min && x == self.max {
                self.max = self.prev(x - 1).expect("self.min != self.max");
            }

            true
        } else {
            debug_assert!(x != self.max);
            false
        }
    }

    pub fn next(&self, x: usize) -> Option<usize> {
        debug_assert!(x < Self::CAPACITY);

        if self.is_empty() || x > self.max {
            return None;
        }
        if x <= self.min {
            return Some(self.min);
        }

        let (ux, lx) = Self::ul(x);
        if let Some(last) = self.lower[ux].last() {
            if lx <= last {
                return Some((ux << Lower::BITS) + self.lower[ux].next(lx).expect("lx <= last"));
            }
        }

        let ux = self.upper.next(ux + 1).expect("self.min < x <= self.max");
        let lx = self.lower[ux].first().expect("self.min < x <= self.max");

        Some((ux << Lower::BITS) + lx)
    }

    pub fn prev(&self, x: usize) -> Option<usize> {
        debug_assert!(x < Self::CAPACITY);

        if self.is_empty() || x < self.min {
            return None;
        }
        let (ux, lx) = Self::ul(x);
        if let Some(first) = self.lower[ux].first() {
            if lx >= first {
                return Some((ux << Lower::BITS) + self.lower[ux].prev(lx).expect("lx >= first"));
            }
        }

        if ux > 0 {
            if let Some(ux) = self.upper.prev(ux - 1) {
                let lx = self.lower[ux].last().expect("self.min <= x < self.max");
                return Some((ux << Lower::BITS) + lx);
            }
        }

        Some(self.min)
    }

    pub fn first(&self) -> Option<usize> {
        (!self.is_empty()).then_some(self.min)
    }

    pub fn last(&self) -> Option<usize> {
        (!self.is_empty()).then_some(self.max)
    }
}

impl<const UPPER_CAPACITY: usize, Upper: InnerVEBTree, Lower: InnerVEBTree> crate::VEBTree
    for VEBTree<UPPER_CAPACITY, Upper, Lower>
where
    [(); UPPER_CAPACITY]: ConditionalHasDeepMaybeUninit,
{
    fn capacity(&self) -> usize {
        Self::capacity()
    }

    #[cfg(feature = "dyn_capacity")]
    fn init(value: &mut <Self as HasDeepMaybeUninit>::AsDeepMaybeUninit) {
        Self::init(value);
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
