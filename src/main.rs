use image::{
	DynamicImage,
	ImageBuffer,
	Rgb,
};

fn main() {
	const W: u32 = 640;
    const H: u32 = 640;
    let mut image = DynamicImage::new_rgb32f(W, H);

	let rgb32f_buffer: &mut ImageBuffer<Rgb<f32>, Vec<f32>> = 
	match image {
        DynamicImage::ImageRgb32F(ref mut buf) => buf,
        _ => unreachable!("Rgb32 image was just created"),
    };

	for pixel in rgb32f_buffer.pixels_mut() {
		pixel[0] = 1.0;
		pixel[1] = 0.0;
		pixel[2] = 0.0;
	}


	match image.into_rgb8().save("MyImage.png") {
		Err(e) => println!("Error saving image: {}", e),
		_ => println!("Image saved"),
	}
}
