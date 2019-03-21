use amethyst::{
    core::math::Vector2,
    ecs::prelude::{Component, DenseVecStorage},
};

use crate::drawing::Pixel;

/// A `Frame` is a component that contains a portion of the screen for easier
/// rendering. We don't create it directly, but instead use the `FrameBuilder`.
#[derive(Debug)]
pub struct Frame {
    position: Vector2<usize>,
    pixels: Vec<Vec<Pixel>>,
}

impl Component for Frame {
    type Storage = DenseVecStorage<Self>;
}

impl Frame {
    /// Returns the height in grid `Pixel`s
    pub fn height(&self) -> usize {
        self.pixels.len()
    }

    /// Returns the width in grid `Pixel`s
    pub fn width(&self) -> usize {
        self.pixels[0].len()
    }

    /// Returns the total size (area) of the grid in `Pixel`s
    pub fn size(&self) -> usize {
        self.height() * self.width()
    }
    /// Writes a `Pixel` at the specified coordinate
    /// 
    /// * `coordinate` - A `Vector2` containing the XY coordinates to which the `Pixel` is written
    /// * `pixel` - The `Pixel` data that is written to the coordinate
    pub fn write_pixel(&mut self, coordinate: Vector2<usize>, pixel: Pixel) {
        self.pixels[coordinate.x][coordinate.y] = pixel;
    }

    /// Return a reference to the `Pixel` at the specified coordinate
    /// 
    /// * `coordinate` - A `Vector2` containing the XY coordinates at which the `Pixel` is read
    pub fn read_pixel(&self, coordinate: Vector2<usize>) -> &Pixel {
        &self.pixels[coordinate.x][coordinate.y]
    }
}

impl Default for FrameBuilder {
    fn default() -> Self {
        FrameBuilder {
            position: Vector2::new(0, 0),
            pixels: vec![],
        }
    }
}

/// Gives us the ability to either build a bunch of empty pixels based on
/// dimensions, or specify the exact pixels with a 2D Vector
pub enum PixelInit {
    /// Create empty sprites by specifying a width and height
    Empty(Vector2<usize>),
    /// Create pixels directly using a 2D vector
    Filled(Vec<Vec<Pixel>>),
}

/// Builder for the `Frame` component.
struct FrameBuilder {
    pixels: Vec<Vec<Pixel>>,
    position: Vector2<usize>,
}

impl FrameBuilder {
    /// Returns a new `FrameBuilder` with default data
    pub fn new() -> Self {
        FrameBuilder::default()
    }
    /// Returns the builder with an initialized array of pixels
    ///
    /// * `init` - Allows us to fill the frame with empty `Pixel`s, or with a
    ///   predefined vector of `Pixel`s
    pub fn initialize(&mut self, init: PixelInit) -> &mut Self {
        // Initialize with data depending on if we want to start empty or not.
        match init {
            PixelInit::Empty(size) => self.pixels = vec![vec![Pixel::default(); size.x]; size.y],
            PixelInit::Filled(pixels) => self.pixels = pixels,
        }
        self
    }

    /// Returns the build with optional initial relative offset to the game screen
    ///
    /// * `position` - A `Vector2` containing two `usize` X and Y coordinates
    ///   from which to offset. Note that this offset is in real pixels, and not
    ///   a grid. This allows for fluid window movement if desired.
    pub fn position(&mut self, position: Vector2<usize>) -> &mut Self {
        self.position = position;
        self
    }
}
