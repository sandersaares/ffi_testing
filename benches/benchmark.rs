use std::{ffi::c_ulong, hint::black_box, sync::Arc};

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

fn time() -> c_ulong {
    unsafe { GetTickCount() }
}

trait Clock {
    fn now(&self) -> c_ulong;
}

struct RealClock {}

impl Clock for RealClock {
    fn now(&self) -> c_ulong {
        time()
    }
}

#[link(name = "kernel32")]
extern "system" {
    fn GetTickCount() -> c_ulong;
}
