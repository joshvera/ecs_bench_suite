use brood::{
    entities,
    query::{filter, result, Views},
    Registry, World, Query,
};
use cgmath::{Matrix4, Vector3};

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

type Registry = Registry!(Transform, Position, Rotation, Velocity);

pub struct Benchmark(World<Registry>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();

        world.extend(entities!((
            Transform(Matrix4::from_scale(1.0)),
            Position(Vector3::unit_x()),
            Rotation(Vector3::unit_x()),
            Velocity(Vector3::unit_x())
        ); 10_000));

        Self(world)
    }

    pub fn run(&mut self) {
        for result!(velocity, position) in self
            .0
            .query(Query::<Views!(&Velocity, &mut Position), filter::None>::new()).iter
        {
            position.0 += velocity.0;
        }
    }
}
