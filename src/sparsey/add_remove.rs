use sparsey::prelude::*;

struct A(f32);
struct B(f32);

pub struct Benchmark(World, Vec<Entity>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();
        world.register::<A>();
        world.register::<B>();

        let entities = world.bulk_create((0..10_000).map(|_| (A(0.0),))).to_vec();

        Self(world, entities)
    }

    pub fn run(&mut self) {
        for &entity in &self.1 {
            self.0.insert(entity, (B(0.0),));
        }

        for &entity in &self.1 {
            self.0.delete::<(B,)>(entity);
        }
    }
}
