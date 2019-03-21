extern crate amethyst;
mod screen;
mod stry;

use crate::screen::Screen;
use crate::stry::Stry;

use amethyst::{
    ecs::Resources,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    // Set up our display and other boilerplate
    amethyst::start_logger(Default::default());

    // Load our display file from our project root and apply
    let display_config_path = format!("{}/resources/display_config.ron", application_root_dir());
    let display_config = DisplayConfig::load(&display_config_path);

    // build everything
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );

    let game_data =
        GameDataBuilder::default().with_bundle(RenderBundle::new(pipe, Some(display_config)))?;
    let mut game = Application::new("./", Stry, game_data)?;
    game.run();

    Ok(())
}
