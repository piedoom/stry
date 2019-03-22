use crate::drawing::frame::Frame;
use amethyst::ecs::Entity;
use amethyst::prelude::{Builder, World};

/// Adds a `Frame` to the `World` and returns the newly attached `Entity`
///
/// * `frame` - the `Frame` that should attach to the world
/// * `world` - a mutable reference to the `World` to attach a frame upon
pub fn add_frame(frame: Frame, world: &mut World) -> Entity {
    world.create_entity().with(frame).build()
}
