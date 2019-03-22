extern crate amethyst;
mod drawing;
mod stry;

use crate::stry::Stry;

use amethyst::{
    core::transform::TransformBundle,
    ecs::Resources,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    // Set up our display and other boilerplate
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_path = app_root.join("assets");
    let display_config_path = app_root.join("resources/display_config.ron");

    // Load our display file from our project root and apply
    let display_config = DisplayConfig::load(&display_config_path);

    // build everything
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(display_config)))?
        .with_bundle(TransformBundle::new())?;
    let mut game = Application::new(assets_path, Stry, game_data)?;
    game.run();

    Ok(())
}
