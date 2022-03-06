use rayon::{ThreadPool, ThreadPoolBuilder};
use sparsey::prelude::*;

struct A(f32);
struct B(f32);
struct C(f32);
struct D(f32);
struct E(f32);

fn ab(mut a: CompMut<A>, mut b: CompMut<B>) {
    (&mut a, &mut b).for_each(|(a, b)| {
        std::mem::swap(&mut a.0, &mut b.0);
    });
}

fn cd(mut c: CompMut<C>, mut d: CompMut<D>) {
    (&mut c, &mut d).for_each(|(c, d)| {
        std::mem::swap(&mut c.0, &mut d.0);
    });
}

fn ce(mut c: CompMut<C>, mut e: CompMut<E>) {
    (&mut c, &mut e).for_each(|(c, e)| {
        std::mem::swap(&mut c.0, &mut e.0);
    });
}

pub struct Benchmark(World, Resources, Schedule, ThreadPool);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();
        world.register::<A>();
        world.register::<B>();
        world.register::<C>();
        world.register::<D>();
        world.register::<E>();

        world.bulk_create((0..10_000).map(|_| (A(0.0),)));
        world.bulk_create((0..10_000).map(|_| (A(0.0), B(0.0))));
        world.bulk_create((0..10_000).map(|_| (A(0.0), B(0.0), C(0.0))));
        world.bulk_create((0..10_000).map(|_| (A(0.0), B(0.0), C(0.0), D(0.0))));
        world.bulk_create((0..10_000).map(|_| (A(0.0), B(0.0), C(0.0), E(0.0))));

        let schedule = Schedule::builder()
            .add_system(ab)
            .add_system(cd)
            .add_system(ce)
            .build();

        let thread_pool = ThreadPoolBuilder::new()
            .num_threads(schedule.max_threads())
            .build()
            .unwrap();

        Self(world, Resources::default(), schedule, thread_pool)
    }

    pub fn run(&mut self) {
        self.2.run_in_thread_pool(&mut self.0, &mut self.1, &self.3);
    }
}
