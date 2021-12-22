use cgmath::*;
use legion::*;
use query::Query;
use storage::PackOptions;

#[derive(Clone, Copy)]
struct Transform(Matrix4<f32>);

#[derive(Clone, Copy)]
struct Position(Vector3<f32>);

#[derive(Clone, Copy)]
struct Rotation(Vector3<f32>);

#[derive(Clone, Copy)]
struct Velocity(Vector3<f32>);

pub struct Benchmark(World, Query<(Read<Velocity>, Write<Position>)>);

impl Benchmark {
    pub fn new() -> Self {
        let options = WorldOptions {
            groups: vec![<(Velocity, Position)>::to_group()],
        };

        let mut world = World::new(options);

        world.extend(
            (
                vec![Transform(Matrix4::from_scale(1.0)); 10_000],
                vec![Position(Vector3::unit_x()); 10_000],
                vec![Rotation(Vector3::unit_x()); 10_000],
                vec![Velocity(Vector3::unit_x()); 10_000],
            )
                .into_soa(),
        );
        world.pack(PackOptions::force());

        let query = <(Read<Velocity>, Write<Position>)>::query();

        Self(world, query)
    }

    pub fn run(&mut self) {
        self.1.for_each_mut(&mut self.0, |(velocity, position)| {
            position.0 += velocity.0;
        });
    }
}
