use brood::{entities, entity, Registry, World};

#[derive(Clone)]
struct A(f32);
struct B(f32);

type Registry = Registry!(A, B);

pub struct Benchmark(World<Registry>, Vec<entity::Identifier>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();

        let entities = world.extend(entities!((A(0.0)); 10_000));

        Self(world, entities)
    }

    pub fn run(&mut self) {
        for entity_identifier in &self.1 {
            self.0.entry(*entity_identifier).unwrap().add(B(0.0));
        }

        for entity_identifier in &self.1 {
            self.0.entry(*entity_identifier).unwrap().remove::<B, _>();
        }
    }
}
