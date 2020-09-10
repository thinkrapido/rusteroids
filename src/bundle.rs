use amethyst::core::SystemBundle;
use amethyst::prelude::World;
use amethyst::core::ecs::DispatcherBuilder;
use amethyst::error::Error;
use crate::systems::MoveAsteroidsSystem;

pub struct RusteroidsBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for RusteroidsBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {

        builder.add(MoveAsteroidsSystem, "asteroid_system", &[]);

        Ok(())
    }
}