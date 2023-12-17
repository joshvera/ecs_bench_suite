use brood::{
    entities,
    query::{filter, Views, Result},
    Registry,
    system::{schedule, schedule::task, System},
    system::Schedule,
    result,
    World,
};

#[derive(Clone)]
struct A(f32);
#[derive(Clone)]
struct B(f32);
#[derive(Clone)]
struct C(f32);
#[derive(Clone)]
struct D(f32);
#[derive(Clone)]
struct E(f32);

type Registry = Registry!(A, B, C, D, E);

struct AB;

impl System for AB {
    type Filter = filter::None;
    type Views<'a> = Views!(&'a mut A, &'a mut B);
    type ResourceViews<'a> = Views!();
    type EntryViews<'a> = Views!();

    fn run<'a, R, S, I, E>(&mut self, query_results: Result<R, S, I, Self::ResourceViews<'a>, Self::EntryViews<'a>, E>)
    where
        R: brood::registry::Registry,
        I: Iterator<Item = Self::Views<'a>>
    {
        for result!(a, b) in query_results.iter {
            std::mem::swap(&mut a.0, &mut b.0);
        }
    }
}

struct CD;

impl System for CD {
    type Views<'a> = Views!(&'a mut C, &'a mut D);
    type Filter = filter::None;
    type ResourceViews<'a> = Views!();
    type EntryViews<'a> = Views!();

    fn run<'a, R, S, I, E>(&mut self, query_results: Result<R, S, I, Self::ResourceViews<'a>, Self::EntryViews<'a>, E>)
    where
        R: brood::registry::Registry + 'a,
        I: Iterator<Item = Self::Views<'a>>,
    {
        for result!(c, d) in query_results.iter {
            std::mem::swap(&mut c.0, &mut d.0);
        }
    }
}

struct CE;

impl System for CE {
    type Views<'a> = Views!(&'a mut C, &'a mut E);
    type Filter = filter::None;
    type ResourceViews<'a> = Views!();
    type EntryViews<'a> = Views!();

    fn run<'a, R, S, I, E>(&mut self, query_results: Result<R, S, I, Self::ResourceViews<'a>, Self::EntryViews<'a>, E>)
    where
        R: brood::registry::Registry + 'a,
        I: Iterator<Item = Self::Views<'a>>,
    {
        for result!(c, e) in query_results.iter {
            std::mem::swap(&mut c.0, &mut e.0);
        }
    }
}


type Schedule = Schedule!(task::System<AB>, task::System<CD>, task::System<CE>);

pub struct Benchmark(
    World<Registry>,
    Schedule
);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();

        world.extend(entities!((A(0.0), B(0.0)); 10_000));
        world.extend(entities!((A(0.0), B(0.0), C(0.0)); 10_000));
        world.extend(entities!((A(0.0), B(0.0), C(0.0), D(0.0)); 10_000));
        world.extend(entities!((A(0.0), B(0.0), C(0.0), E(0.0)); 10_000));

        let schedule = schedule!(task::System(AB), task::System(CD), task::System(CE));

        Self(world, schedule)
    }

    pub fn run(&mut self) {
        self.0.run_schedule(&mut self.1);
    }
}
