use criterion::{
    black_box, criterion_group, criterion_main, measurement::Measurement, BatchSize,
    BenchmarkGroup, BenchmarkId, Criterion,
};
use rand::{
    distributions::{Bernoulli, Uniform},
    prelude::StdRng,
    Rng, SeedableRng,
};
use std::{collections::BTreeSet, time::Duration};

trait VEBOperations {
    fn insert(&mut self, x: usize) -> bool;
    fn remove(&mut self, x: usize) -> bool;
    fn contains(&self, x: usize) -> bool;
    fn next(&self, x: usize) -> Option<usize>;
    fn prev(&self, x: usize) -> Option<usize>;
}

impl<T: flat_veb::VEBTree> VEBOperations for T {
    fn insert(&mut self, x: usize) -> bool {
        self.insert(x)
    }

    fn remove(&mut self, x: usize) -> bool {
        self.remove(x)
    }

    fn contains(&self, x: usize) -> bool {
        self.contains(x)
    }

    fn next(&self, x: usize) -> Option<usize> {
        self.next(x)
    }

    fn prev(&self, x: usize) -> Option<usize> {
        self.prev(x)
    }
}
struct BTreeSetWrapper(BTreeSet<u32>);

impl VEBOperations for BTreeSetWrapper {
    fn insert(&mut self, x: usize) -> bool {
        self.0.insert(x as u32)
    }

    fn remove(&mut self, x: usize) -> bool {
        self.0.remove(&(x as u32))
    }

    fn contains(&self, x: usize) -> bool {
        self.0.contains(&(x as u32))
    }

    fn next(&self, x: usize) -> Option<usize> {
        Some(*self.0.range(x as u32..).next()? as usize)
    }

    fn prev(&self, x: usize) -> Option<usize> {
        Some(*self.0.range(..=x as u32).next_back()? as usize)
    }
}

fn for_all_widths<'a, M: Measurement, Tree, Ret>(
    mut group: BenchmarkGroup<'a, M>,
    mut maker: impl FnMut(&mut StdRng, usize) -> Tree,
    mut test: impl FnMut(&mut Tree, usize) -> Ret,
) {
    group.warm_up_time(Duration::from_millis(1000));
    group.measurement_time(Duration::from_millis(2000));

    for bits in 4..=30 {
        let capacity = 1 << bits;
        let distr = Uniform::from(0..capacity);

        let mut rng = StdRng::seed_from_u64(0);

        let mut s = maker(&mut rng, bits);
        group.bench_function(BenchmarkId::from_parameter(bits), |b| {
            b.iter_batched(
                || rng.sample(distr),
                |x| test(&mut s, x),
                BatchSize::SmallInput,
            );
        });
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let distr = Bernoulli::from_ratio(1, 2).unwrap();

    let veb_maker = |rng: &mut StdRng, bits| {
        let mut s = flat_veb::new_with_bits(bits);
        for x in 0..1 << bits {
            if rng.sample(&distr) {
                s.insert(x);
            }
        }
        s
    };

    let btree_maker = |rng: &mut StdRng, bits: usize| {
        let mut s = BTreeSetWrapper(BTreeSet::new());
        for x in 0..1 << bits {
            if rng.sample(&distr) {
                s.insert(x);
            }
        }
        s
    };

    for_all_widths(
        c.benchmark_group(format!("insert-veb")),
        veb_maker,
        |s, x| black_box(s.insert(x)),
    );
    for_all_widths(
        c.benchmark_group(format!("remove-veb")),
        veb_maker,
        |s, x| black_box(s.remove(x)),
    );
    for_all_widths(
        c.benchmark_group(format!("contains-veb")),
        veb_maker,
        |s, x| black_box(s.contains(x)),
    );
    for_all_widths(c.benchmark_group(format!("next-veb")), veb_maker, |s, x| {
        black_box(s.next(x))
    });
    for_all_widths(c.benchmark_group(format!("prev-veb")), veb_maker, |s, x| {
        black_box(s.prev(x))
    });

    for_all_widths(
        c.benchmark_group(format!("insert-btree")),
        btree_maker,
        |s, x| black_box(s.insert(x)),
    );
    for_all_widths(
        c.benchmark_group(format!("remove-btree")),
        btree_maker,
        |s, x| black_box(s.remove(x)),
    );
    for_all_widths(
        c.benchmark_group(format!("contains-btree")),
        btree_maker,
        |s, x| black_box(s.contains(x)),
    );
    for_all_widths(
        c.benchmark_group(format!("next-btree")),
        btree_maker,
        |s, x| black_box(s.next(x)),
    );
    for_all_widths(
        c.benchmark_group(format!("prev-btree")),
        btree_maker,
        |s, x| black_box(s.prev(x)),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
