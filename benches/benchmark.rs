use ::optsort::output::optsort::optsort7;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use optsort::output::optsort;
use rand::Rng;

fn rand_arr() -> [usize; 7] {
    let mut rng = rand::thread_rng();
    // let range = 0..100;
    [
        rng.gen_range(0..100000),
        rng.gen_range(0..100000),
        rng.gen_range(0..100000),
        rng.gen_range(0..100000),
        rng.gen_range(0..100000),
        rng.gen_range(0..100000),
        rng.gen_range(0..100000),
    ]
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("builtin", |b| {
        b.iter(|| {
            let mut arr = rand_arr();
            arr.sort();
            black_box(arr)
        })
    });

    c.bench_function("optsort7", |b| {
        b.iter(|| {
            let arr = rand_arr();
            optsort7(black_box(arr))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
