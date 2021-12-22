use cgmath::*;
use legion::*;

#[derive(Clone, Copy)]
struct Transform(Matrix4<f32>);

#[derive(Clone, Copy)]
struct Position(Vector3<f32>);

#[derive(Clone, Copy)]
struct Rotation(Vector3<f32>);

#[derive(Clone, Copy)]
struct Velocity(Vector3<f32>);

pub struct Benchmark;

impl Benchmark {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self) {
        let mut world = World::default();

        world.extend(
            (
                vec![Transform(Matrix4::from_scale(1.0)); 10_000],
                vec![Position(Vector3::unit_x()); 10_000],
                vec![Rotation(Vector3::unit_x()); 10_000],
                vec![Velocity(Vector3::unit_x()); 10_000],
            )
                .into_soa(),
        );
    }
}
