use cgmath::*;
use cgmath::Transform as _;
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
            .add_group(<(Transform, Position, Rotation,Velocity)>::group())
            .add_group(<(Position, Transform)>::group())
            .build();

        let mut world = World::with_layout(&layout);

        world.bulk_create((0..1_000).map(|_| {
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
        let resources = Resources::default();
        sparsey::run(&self.0, &resources, transform_sprites);
    }
}

fn transform_sprites(mut pos: CompMut<Position>, mut transforms: CompMut<Transform>) {
    (&mut pos, &mut transforms).for_each(|(pos, mat)| {
        for _ in 0..100 {
            mat.0 = mat.0.invert().unwrap();
        }
        pos.0 = mat.0.transform_vector(pos.0);
    });

}
