use std::{hint::black_box, sync::Arc, time::SystemTime};

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

criterion_group!(benches, price_of_indirection,);
criterion_main!(benches);

fn price_of_indirection(c: &mut Criterion) {
    let mut group = c.benchmark_group("price_of_indirection");

    group.bench_function("raw", |b| {
        b.iter(|| {
            let _ = black_box(time());
        })
    });

    group.bench_function("clock", |b| {
        b.iter_batched_ref(
            || RealClock {},
            |clock| _ = black_box(clock.now()),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("arc_clock", |b| {
        b.iter_batched_ref(
            || Arc::new(RealClock {}),
            |clock| _ = black_box(clock.now()),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn time() -> SystemTime {
    SystemTime::now()
}

trait Clock {
    fn now(&self) -> SystemTime;
}

struct RealClock {}

impl Clock for RealClock {
    fn now(&self) -> SystemTime {
        time()
    }
}
