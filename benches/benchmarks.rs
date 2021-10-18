use criterion::*;

macro_rules! benchmark {
    ($benchmark:ident; $($ecs:ident),+) => {
        fn $benchmark(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($benchmark));
            $(
                group.bench_function(stringify!($ecs), |b| {
                    let mut bench = ecs_bench_suite::$ecs::$benchmark::Benchmark::new();
                    b.iter(move || bench.run());
                });
            )+
        }
    };
}

benchmark!(simple_insert; bevy, hecs, legion, planck_ecs, shipyard, specs);
benchmark!(simple_iter; bevy, hecs, legion, legion_packed, planck_ecs, shipyard, specs);
benchmark!(frag_iter; bevy, hecs, legion, planck_ecs, shipyard, specs);
benchmark!(schedule; bevy, legion, legion_packed, planck_ecs, shipyard, specs);
benchmark!(heavy_compute; bevy, legion, legion_packed, shipyard, specs);
benchmark!(add_remove; bevy, hecs, legion, planck_ecs, shipyard, specs);
benchmark!(serialize_text; hecs);
benchmark!(serialize_binary; hecs);

criterion_group!(
    benchmarks,
    simple_insert,
    simple_iter,
    frag_iter,
    schedule,
    heavy_compute,
    add_remove,
    serialize_text,
    serialize_binary,
);
criterion_main!(benchmarks);
