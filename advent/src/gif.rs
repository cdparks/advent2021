use image::codecs::gif::{GifEncoder, Repeat};
use image::imageops;
use image::Delay;
pub use image::Frame;
use image::{Rgba, RgbaImage};
use std::fs::File;
use std::time::Duration;

/// Write frames to file as a gif.
pub fn write<F>(filename: &str, frames: F)
where
    F: IntoIterator<Item = Frame>,
{
    let gif = File::create(filename).unwrap();
    let mut encoder = GifEncoder::new(gif);
    encoder.set_repeat(Repeat::Infinite).unwrap();
    encoder.encode_frames(frames).unwrap();
}

#[derive(Debug, Clone)]
/// Wrapper for RgbaImage
pub struct Image {
    image: RgbaImage,
}

impl Image {
    /// Create an image with the specified dimensions.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            image: RgbaImage::new(width as u32, height as u32),
        }
    }

    /// Dimensions of the image
    pub fn dimension(&self) -> (usize, usize) {
        (self.image.width() as usize, self.image.height() as usize)
    }

    /// Create a new image from an RgbaImage
    pub fn from(image: RgbaImage) -> Self {
        Self { image }
    }

    /// Resize the image using nearest-neighbor
    pub fn resize(&self, width: usize, height: usize) -> Self {
        Self::from(imageops::resize(
            &self.image,
            width as u32,
            height as u32,
            imageops::FilterType::Nearest,
        ))
    }

    /// Consume the image to create a frame.
    pub fn frame(self, millis: u64) -> Frame {
        Frame::from_parts(
            self.image,
            0,
            0,
            Delay::from_saturating_duration(Duration::from_millis(millis)),
        )
    }

    /// Write fully opaque pixel to coordinates
    pub fn set(&mut self, x: usize, y: usize, pixel: [u8; 3]) {
        let [r, g, b] = pixel;
        self.image.put_pixel(x as u32, y as u32, Rgba([r, g, b, 1]));
    }

    /// Write black pixel to coordinates
    pub fn black(&mut self, x: usize, y: usize) {
        self.set(x, y, [0, 0, 0])
    }
}
