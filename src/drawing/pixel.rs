use amethyst::renderer::Sprite;

/// Represents a sprite "pixel" to which a sprite can be written
#[derive(Debug, Clone, Default)]
pub struct Pixel {
    sprite: Option<Sprite>,
}