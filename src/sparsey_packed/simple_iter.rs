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

        world.bulk_create((0..10_000).map(|_| {
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
        let mut positions = self.0.borrow_mut::<Position>();
        let velocities = self.0.borrow::<Velocity>();

        (&mut positions, &velocities).for_each(|(position, velocity)| {
            position.0 += velocity.0;
        });
    }
}
