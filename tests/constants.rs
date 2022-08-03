use flat_veb::{InnerVEBTree, SizedVEBTree};

#[test]
fn constants_are_correct() {
    macro_rules! check_bits {
        ($n:expr, T T T T $($tail:tt)*) => {
            assert_eq!(SizedVEBTree::<{ $n }>::BITS, $n);
            assert_eq!(SizedVEBTree::<{ $n }>::CAPACITY, 1 << $n);
            check_bits! {
                $n + 1, T T T $($tail)*
            }
        };
        ($n:expr, T T T) => {};
    }
    check_bits! {4,
        T T T T T T T T
        T T T T T T T T
        T T T T T T T T
        T T T T T T T T
        T T T T T T T T
        T T T T T T T T
        T T T T T T T T
        T
    }

    let t = flat_veb::new_with_capacity(1 << 30);
    assert_eq!(t.capacity(), 1 << 30);
}
