mod geometry;
mod hit;
mod material;

use std::ops::RangeInclusive;

pub use self::{geometry::*, hit::*, material::*};
use crate::{ray::Ray, vector::Point};

pub struct Object {
    geometry: ObjectGeometry,
    material: ObjectMaterial,
}

impl Object {
    pub fn new_sphere(center: Point, radius: f64, material: ObjectMaterial) -> Self {
        Self {
            geometry: ObjectGeometry::Sphere { center, radius },
            material,
        }
    }

    pub fn hit<'a>(
        &self,
        ray: &'a Ray,
        ray_length_min_max: RangeInclusive<f64>,
    ) -> Option<HitRecord<'a>> {
        self.geometry.hit(ray, ray_length_min_max).map(
            |(distance_from_ray, point_of_intersection, normal)| {
                HitRecord::new(
                    ray,
                    distance_from_ray,
                    point_of_intersection,
                    normal,
                    self.material,
                )
            },
        )
    }
}
