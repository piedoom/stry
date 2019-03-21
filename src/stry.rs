use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage, Resources};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle,
    Texture, TextureMetadata,
};

pub const ARENA_HEIGHT: f32 = 1024.0;
pub const ARENA_WIDTH: f32 = 1024.0;

pub struct Stry;

use crate::drawing::screen::Screen;

impl SimpleState for Stry {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // set up and attach the camera to our scene
        initialise_camera(world);
        initialize_screen(world);
        dbg!(&*world.read_resource::<Screen>());
    }
}

fn initialize_screen(world: &mut World) {
    // set up resources
    world.add_resource(Screen::default());
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}
