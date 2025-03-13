use anyhow::Context;
use image::buffer::ConvertBuffer;
use image::{Rgb32FImage, RgbImage};

const W: u32 = 640;
const H: u32 = 640;

fn main() -> anyhow::Result<()> {
    let mut image = Rgb32FImage::new(W, H);

    for pixel in image.pixels_mut() {
        pixel[0] = 1.0;
        pixel[1] = 0.0;
        pixel[2] = 0.0;
    }

    let image: RgbImage = image.convert();
    image.save("MyImage.png").context("Error saving image")?;

    Ok(())
}
