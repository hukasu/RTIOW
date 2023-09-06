use crate::vector::Colour;

fn format_pixel(pixel: &Colour) -> String {
    format!(
        "{:.0} {:.0} {:.0}",
        (pixel[0] * 256.).clamp(0., 255.),
        (pixel[1] * 256.).clamp(0., 255.),
        (pixel[2] * 256.).clamp(0., 255.)
    )
}

#[must_use]
pub fn image_to_ppm(image: &crate::Image) -> Box<str> {
    let (width, height) = image.get_dimensions();
    let header = format!("P3\n{width} {height}\n255");
    let body = image
        .get_pixels()
        .iter()
        .map(format_pixel)
        .collect::<Vec<_>>()
        .join("\n");
    format!("{header}\n{body}").into_boxed_str()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn image_to_ppm_test() {
        const SIZE: usize = 256;
        let mut image = crate::Image::new(SIZE, SIZE);
        for (idx, pixel) in image.get_pixels_mut().iter_mut().enumerate() {
            let x = idx % SIZE;
            let y = idx / SIZE;
            *pixel = Colour::new(x as f64 / SIZE as f64, y as f64 / SIZE as f64, 0.);
        }
        let ppm = image_to_ppm(&image);
        std::fs::write("gradient.ppm", ppm.as_bytes()).expect("Expect contents written to file");
    }
}
