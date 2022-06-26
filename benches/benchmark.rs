use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ninja_tools::data_structure::bit::BIT;
use ninja_tools::graph::simple_graph::SimpleGraph;

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

fn min_dists_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph");
    group.sample_size(10);
    group.bench_function("graph min_dist bench", |b| {
        b.iter(|| {
            let mut graph = SimpleGraph::new(3000, false);
            for i in 0..3000 {
                graph.add_edge(black_box(i), i * 2 % 3000, 1usize);
            }
            for i in 0..3000 {
                graph.add_edge(black_box(i), (i + 200) % 3000, 1usize);
            }

            graph.all_min_dists();
        })
    });
}

criterion_group!(benches, bit_bench, min_dists_bench);
criterion_main!(benches);
