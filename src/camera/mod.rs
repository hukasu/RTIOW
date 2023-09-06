mod builder;
pub use builder::*;

use crate::{
    object_storage::ObjectStorage,
    ray::Ray,
    vector::{Colour, Direction, Point},
    Image,
};

#[derive(Debug)]
/// A Camera object representend by it's position in space `center`, the direction it's pointing towards `forward`,
/// where its `up` is, and the dimensions of the sensor (represented by `image_width` and `image_height`).
/// The number of samples that will be collected by each pixel is represented by `shutter_lenghth`. The focal length
/// is taken from the length of the `forward` direction. The view angle is taken from the `up` length.
pub struct Camera {
    center: Point,
    forward: Direction,
    up: Direction,
    left: Direction,
    sensor_width: usize,
    sensor_height: usize,
    focal_distance: f64,
    aperture: f64,
    field_of_view: f64,
    shutter_length: usize,
    max_ray_depth: usize,
}

impl Camera {
    /// Creates a Camera builder
    pub fn builder() -> CameraBuilder<InputPosition> {
        CameraBuilder::new(Self {
            center: Point::default(),
            forward: Direction::default(),
            up: Direction::default(),
            left: Direction::default(),
            sensor_width: usize::default(),
            sensor_height: usize::default(),
            focal_distance: f64::default(),
            aperture: f64::default(),
            field_of_view: f64::default(),
            shutter_length: usize::default(),
            max_ray_depth: usize::default(),
        })
    }

    pub fn capture_image(&self, scene: &impl ObjectStorage) -> Image {
        let mut buffer: Box<[Colour]> =
            vec![Colour::default(); self.sensor_width * self.sensor_height].into_boxed_slice();

        let scale = 1.0 / self.shutter_length as f64;

        for (idx, pixel) in buffer.iter_mut().enumerate() {
            let horizontal_pixel_id = idx % self.sensor_width;
            let vertical_pixel_id = idx / self.sensor_width;

            for _ in 0..self.shutter_length {
                let pixel_ray = self.get_ray_for_pixel(horizontal_pixel_id, vertical_pixel_id);
                *pixel += Self::trace_ray(pixel_ray.unit_ray(), scene, self.max_ray_depth);
            }
        }

        Image {
            pixels: buffer
                .iter()
                .map(|color| (*color * scale).linear_to_gamma())
                .collect(),
            width: self.sensor_width,
            height: self.sensor_height,
        }
    }

    fn generate_jitter() -> (f64, f64) {
        (
            (rand::random::<f64>() * 2.) - 1.,
            (rand::random::<f64>() * 2.) - 1.,
        )
    }

    fn get_ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let apparent_aspect_ration = self.sensor_width as f64 / self.sensor_height as f64;

        let vertical_field_of_view =
            (self.field_of_view.to_radians() / 2.).tan() * self.focal_distance;
        let horizontal_field_of_view = vertical_field_of_view * apparent_aspect_ration;

        let (jitter_x, jitter_y) = Self::generate_jitter();
        let horizontal_offset =
            (((x as f64 + 0.5 + jitter_x) / self.sensor_width as f64) * 2.) - 1.;
        let vertical_offset = (((y as f64 + 0.5 + jitter_y) / self.sensor_height as f64) * 2.) - 1.;

        let (jitter_x, jitter_y) = Self::generate_jitter();
        let aperture_source = self.center
            + (self.up * self.aperture * jitter_y)
            + (self.left * self.aperture * jitter_x);

        // Adding `up` and `left` result on the top-left (0, 0) pixel,
        // so to iterate from top-left to bottom-right, the offsets calculated above
        // need to be flipped
        let viewport_target = self.center
            + (self.forward * self.focal_distance)
            + (self.up * vertical_field_of_view * -vertical_offset)
            + (self.left * horizontal_field_of_view * -horizontal_offset);

        Ray::new(
            aperture_source,
            aperture_source.point_towards(viewport_target).unit_vector(),
        )
    }

    fn trace_ray(ray: Ray, scene: &impl ObjectStorage, depth: usize) -> Colour {
        if depth == 0 {
            Colour::default()
        } else if let Some(hit) = scene.find_intersection(&ray, 0.001..=f64::INFINITY) {
            let scatter = hit.scatter();
            scatter.attenuation * Self::trace_ray(scatter.scattered, scene, depth - 1)
        } else {
            let unit_direction = ray.direction();
            let a = 0.5 * (unit_direction[1] + 1.);
            (1. - a) * Colour::new(1., 1., 1.) + a * Colour::new(0.5, 0.7, 1.0)
        }
    }
}
