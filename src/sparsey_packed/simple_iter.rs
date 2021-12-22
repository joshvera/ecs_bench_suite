use cgmath::*;
use sparsey::prelude::*;

#[derive(Clone, Copy)]
struct Transform(Matrix4<f32>);

#[derive(Clone, Copy)]
struct Position(Vector3<f32>);

#[derive(Clone, Copy)]
struct Rotation(Vector3<f32>);

#[derive(Clone, Copy)]
struct Velocity(Vector3<f32>);

pub struct Benchmark(World);

impl Benchmark {
    pub fn new() -> Self {
        let layout = Layout::builder()
            .add_group(<(Position, Velocity)>::group())
            .build();

        let mut world = World::with_layout(&layout);
        world.register::<Transform>();
        world.register::<Position>();
        world.register::<Rotation>();
        world.register::<Velocity>();

        world.create_entities((0..10_000).map(|_| {
            (
                Transform(Matrix4::<f32>::from_angle_x(Rad(1.2))),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            )
        }));

        Self(world)
    }

    pub fn run(&mut self) {
        let (mut positions, velocities) = self.0.borrow::<(CompMut<Position>, Comp<Velocity>)>();

        (&mut positions, &velocities)
            .iter()
            .for_each(|(mut position, velocity)| {
                position.0 += velocity.0;
            });
    }
}
