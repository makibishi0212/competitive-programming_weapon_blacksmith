use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ninja_tools::data_structure::BIT::BIT;

fn bit_bench(c: &mut Criterion) {
    c.bench_function("bit bench", |b| {
        b.iter(|| {
            let mut bit = BIT::new(vec![1; 1000000]);
            for i in 0..1000000 {
                bit.add(i, 1);
                bit.query(black_box(i), 1000000);
            }
        })
    });
}

criterion_group!(benches, bit_bench);
criterion_main!(benches);
