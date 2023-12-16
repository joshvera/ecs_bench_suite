use bevy_ecs::prelude::*;
use cgmath::*;

#[derive(Clone, Copy, Component)]
#[component(storage = "SparseSet")]
struct Transform(Matrix4<f32>);

#[derive(Clone, Copy, Component)]
#[component(storage = "SparseSet")]
struct Position(Vector3<f32>);

#[derive(Clone, Copy, Component)]
#[component(storage = "SparseSet")]
struct Rotation(Vector3<f32>);

#[derive(Clone, Copy, Component)]
#[component(storage = "SparseSet")]
struct Velocity(Vector3<f32>);

pub struct Benchmark;

impl Benchmark {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self) {
        let mut world = World::new();
        world.spawn_batch((0..10_000).map(|_| {
            (
                Transform(Matrix4::from_scale(1.0)),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            )
        }));
    }
}
