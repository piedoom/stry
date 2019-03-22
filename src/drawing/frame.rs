use amethyst::{
    core::math::Vector2,
    ecs::prelude::{Component, DenseVecStorage},
};

use crate::drawing::Pixel;

/// A `Frame` is a component that contains a portion of the screen for easier
/// rendering. We don't create it directly, but instead use the `FrameBuilder`.
#[derive(Debug)]
pub struct Frame {
    pub position: Vector2<usize>,
    pub size: Vector2<usize>,
    pixels: Vec<Pixel>,
}

impl Component for Frame {
    type Storage = DenseVecStorage<Self>;
}

impl Frame {
    /// Returns the total size (area) of the grid in `Pixel`s
    pub fn area(&self) -> usize {
        self.pixels.len()
    }
    /// Writes a `Pixel` at the specified coordinate
    ///
    /// * `coordinate` - A `Vector2` containing the XY coordinates to which the `Pixel` is written
    /// * `pixel` - The `Pixel` data that is written to the coordinate
    pub fn write_pixel(&mut self, coordinate: Vector2<usize>, pixel: Pixel) {
        let index = self.coordinate_to_index(coordinate);
        self.pixels[index] = pixel;
    }

    /// Return a reference to the `Pixel` at the specified coordinate
    ///
    /// * `coordinate` - A `Vector2` containing the XY coordinates at which the `Pixel` is read
    pub fn read_pixel(&self, coordinate: Vector2<usize>) -> &Pixel {
        let index = self.coordinate_to_index(coordinate);
        &self.pixels[index]
    }

    /// Returns the index of a one-dimensional `Vec`, given a coordinate. Does not check out of bounds
    fn coordinate_to_index(&self, coordinate: Vector2<usize>) -> usize {
        coordinate.x + (self.size.x * coordinate.y)
    }
}

impl Default for FrameBuilder {
    fn default() -> Self {
        FrameBuilder {
            position: Vector2::new(0, 0),
            pixels: vec![],
            size: Vector2::new(1, 1),
        }
    }
}

/// Builder for the `Frame` component.
pub struct FrameBuilder {
    pixels: Vec<Pixel>,
    position: Vector2<usize>,
    size: Vector2<usize>,
}

impl FrameBuilder {
    /// Returns the builder with an initialized array of pixels
    ///
    /// * `size` - The width and height of our frame, where (1,1) is a 1x1 grid.
    pub fn initialize(size: Vector2<usize>) -> Self {
        let mut builder = FrameBuilder::default();
        builder.pixels = vec![Pixel::default(); size.x * size.y];
        builder
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

    /// Builds and returns the `Frame`
    pub fn finalize(self) -> Frame {
        Frame {
            position: self.position,
            size: self.size,
            pixels: self.pixels,
        }
    }
}
