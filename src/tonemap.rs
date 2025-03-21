use image::Rgb32FImage;
use rayon::iter::ParallelIterator;

// TODO: Change this to the name of the actual tonemapping function
pub fn tonemap(image: &mut Rgb32FImage) {
    image.par_pixels_mut().for_each(|pixel| {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];

        // Step 1: Compute luminance
        let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;

        // Step 2: Compress luminance
        let compressed_luminance = luminance / (luminance + 1.0);

        // Step 3: Helmholtz-Kohlrausch effect (simplified)
        let saturation = if luminance > 0.0 {
            let max_channel = r.max(g).max(b);
            let min_channel = r.min(g).min(b);
            (max_channel - min_channel) / max_channel
        } else {
            0.0
        };
        let hk_boost = 1.0 + 0.2 * saturation;
        let adjusted_luminance = (compressed_luminance * hk_boost).clamp(0.0, 1.0);

        // Step 4: Scale colors to preserve ratios
        let scale = if luminance > 0.0 {
            adjusted_luminance / luminance
        } else {
            1.0
        };

        // Update the pixel
        pixel[0] = (r * scale).clamp(0.0, 1.0);
        pixel[1] = (g * scale).clamp(0.0, 1.0);
        pixel[2] = (b * scale).clamp(0.0, 1.0);
    })
}
