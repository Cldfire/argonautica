extern crate a2;
#[macro_use]
extern crate criterion;

use a2::Hasher;
use criterion::Criterion;

const SAMPLE_SIZE: usize = 100;

fn bench_fast_but_insecure(c: &mut Criterion) {
    let mut hasher = Hasher::fast_but_insecure();
    hasher.with_password("some document");
    c.bench_function("bench_fast_but_insecure", move |b| {
        b.iter(|| {
            hasher.hash().unwrap();
        })
    });
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(SAMPLE_SIZE);
    targets = bench_fast_but_insecure,
}
criterion_main!(benches);
