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

benchmark!(add_remove; bevy, bevy_sparse_set, brood, hecs, legion, planck_ecs, shipyard, sparsey, specs);
benchmark!(frag_iter; bevy, bevy_sparse_set, brood, hecs, legion, planck_ecs, shipyard, sparsey, specs);
benchmark!(heavy_compute; bevy, bevy_sparse_set, brood, legion, legion_packed, shipyard, sparsey, sparsey_packed, specs);
benchmark!(schedule; bevy, bevy_sparse_set, brood, legion, legion_packed, planck_ecs, shipyard, sparsey, sparsey_packed, specs);
benchmark!(serialize_binary; hecs, brood);
benchmark!(serialize_text; hecs, brood);
benchmark!(simple_insert; bevy, bevy_sparse_set, brood, hecs, legion, planck_ecs, shipyard, sparsey, specs);
benchmark!(simple_iter; bevy, bevy_sparse_set, brood, hecs, legion, legion_packed, planck_ecs, shipyard, sparsey, sparsey_packed, specs);

criterion_group!(
    benchmarks,
    add_remove,
    frag_iter,
    heavy_compute,
    schedule,
    serialize_binary,
    serialize_text,
    simple_insert,
    simple_iter,
);
criterion_main!(benchmarks);
