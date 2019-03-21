/// Represents a sprite "pixel" to which a sprite can be written
type Pixel = bool;

#[derive(Debug)]
pub struct Screen {
    pixels: Vec<Vec<Pixel>>,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        Screen {
            pixels: vec![vec![false; height]; width],
        }
    }

    /// Write to an X,Y position with a Pixel
    pub fn write_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        self.pixels[x][y] = pixel;
    }

    pub fn read_pixel(&self, x: usize, y: usize) -> Pixel {
        self.pixels[x][y]
    }

    pub fn height(&self) -> usize {
        self.pixels.len()
    }

    pub fn width(&self) -> usize {
        self.pixels[0].len()
    }

    pub fn size(&self) -> usize {
        self.height() * self.width()
    }
}
