macro_rules! make_tests {
    ($name:ident, $n:literal) => {
        mod $name {
            use flat_veb::{InnerVEBTree, SizedVEBTree, VEBTree};

            type T = SizedVEBTree<$n>;

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

macro_rules! make_many_tests {
        ($($name:ident $n:literal)*) => {
            $(make_tests!{$name, $n})*
        }
    }

make_many_tests!(
    size_4 4
    size_5 5
    size_6 6
    size_7 7
    size_8 8
    size_9 9
    size_10 10
    size_15 15
    size_20 20
);
