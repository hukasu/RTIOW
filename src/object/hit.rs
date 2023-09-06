use crate::{
    ray::Ray,
    vector::{Colour, Direction, Point},
};

use super::ObjectMaterial;

pub struct RayScatter {
    pub attenuation: Colour,
    pub scattered: Ray,
}

pub struct HitRecord<'a> {
    pub intersecting_ray: &'a Ray,
    pub distance_from_ray: f64,
    pub point_of_intersection: Point,
    pub normal: Direction,
    pub material: ObjectMaterial,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        intersecting_ray: &'a Ray,
        distance_from_ray: f64,
        point_of_intersection: Point,
        normal: Direction,
        material: ObjectMaterial,
    ) -> Self {
        let front_face = intersecting_ray.direction().dot(normal) < 0.;
        HitRecord {
            intersecting_ray,
            distance_from_ray,
            point_of_intersection,
            normal: if front_face { normal } else { -normal },
            material,
            front_face,
        }
    }

    pub fn scatter(self) -> RayScatter {
        self.material.scatter(&self)
    }
}
