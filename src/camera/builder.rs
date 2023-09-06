use crate::vector::{Direction, Point};

use super::Camera;

pub struct InputPosition;
pub struct InputSensor;
pub struct InputLens;
pub struct InputComplete;

pub struct CameraBuilder<T> {
    camera: Camera,
    state: std::marker::PhantomData<T>,
}

impl CameraBuilder<InputPosition> {
    /// Creates a CameraBuilder
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            state: std::marker::PhantomData,
        }
    }

    /// Inputs `center`, `forward`, and `up` of a Camera</br>
    /// # Parameters
    /// `center`: Point in space where the camera is present</br>
    /// `forward`: Direction where the camera is pointing towards</br>
    /// `up`: Direction where the top of the camera is pointing towards</br>
    /// # Note
    /// When `up` is not perpendicular to `forward` the image will have a tilt-shift-like effect.
    pub fn input_position(
        mut self,
        center: Point,
        forward: Direction,
        up: Direction,
    ) -> CameraBuilder<InputSensor> {
        self.camera.center = center;
        self.camera.forward = forward.unit_vector();
        self.camera.up = up.unit_vector();
        self.camera.left = up.cross(forward).unit_vector();

        CameraBuilder {
            camera: self.camera,
            state: std::marker::PhantomData::<InputSensor>,
        }
    }
}

impl CameraBuilder<InputSensor> {
    /// Inputs `sensor_width`, `sensor_height`, `shutter_length`, and `max_ray_depth` of a Camera</br>
    /// # Parameters
    /// `sensor_width`: Width in pixels of the camera sensor</br>
    /// `sensor_height`: Height in pixels of the camera sensor</br>
    /// `shutter_length`: Number of samples to take when rendering</br>
    /// `max_depth`: Number of bounces a ray can do</br>
    pub fn input_sensor(
        mut self,
        sensor_width: usize,
        sensor_height: usize,
        shutter_length: usize,
        max_ray_depth: usize,
    ) -> CameraBuilder<InputLens> {
        self.camera.sensor_width = sensor_width;
        self.camera.sensor_height = sensor_height;
        self.camera.shutter_length = shutter_length;
        self.camera.max_ray_depth = max_ray_depth;

        CameraBuilder {
            camera: self.camera,
            state: std::marker::PhantomData::<InputLens>,
        }
    }
}

impl CameraBuilder<InputLens> {
    /// Inputs `focal_distance`, `aperture`, and `field_of_view` of a Camera</br>
    /// # Parameters
    /// `focal_distance`: Distance that the camera will focus, objects closer or farther will be blurred</br>
    /// `aperture`: Determinates the size of the aperture, higher values will cause objects out of
    /// focus to be blurrier. Will now darken the image like in a physical camera.</br>
    /// `field_of_view`: Vertical view angle, clamped between (0.0, 180.)</br>
    pub fn input_lens(
        mut self,
        focal_distance: f64,
        aperture: f64,
        field_of_view: f64,
    ) -> CameraBuilder<InputComplete> {
        self.camera.focal_distance = focal_distance.max(0.);
        self.camera.aperture = aperture.max(0.);
        self.camera.field_of_view = field_of_view.clamp(0.1, 179.9);

        CameraBuilder {
            camera: self.camera,
            state: std::marker::PhantomData::<InputComplete>,
        }
    }
}

impl CameraBuilder<InputComplete> {
    pub fn build(self) -> Camera {
        self.camera
    }
}
