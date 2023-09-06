pub mod camera;
pub mod object;
pub mod object_storage;
pub mod ppm;
pub mod ray;
pub mod vector;

use vector::Colour;

pub struct Image {
    pixels: Box<[Colour]>,
    width: usize,
    height: usize,
}

impl Image {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![Colour::default(); width.saturating_mul(height)].into_boxed_slice(),
            width,
            height,
        }
    }

    pub fn get_pixels(&self) -> &[Colour] {
        &self.pixels
    }

    pub fn get_pixels_mut(&mut self) -> &mut [Colour] {
        &mut self.pixels
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}
