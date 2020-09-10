use amethyst::core::ecs::{System, ReadStorage, WriteStorage, Read, Join};
use amethyst::derive::SystemDesc;
use amethyst::prelude::World;
use amethyst::core::{Transform, Time};

pub struct Asteroid {
    velocity: [f32; 2],
    radius: f32,
}

#[derive(SystemDesc)]
pub struct MoveAsteroidsSystem;

impl <'a> System<'a> for MoveAsteroidsSystem {
    type SystemData = (
        ReadStorage<'a, Asteroid>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (asteroids, mut locals, time): Self::SystemData) {
        for (asteroid, local) in (&asteroids, &mut locals).join() {
            local.prepend_translation_x(asteroid.velocity[0] * time.delta_seconds());
            local.prepend_translation_y(asteroid.velocity[1] * time.delta_seconds());
        }
    }
}

