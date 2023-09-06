use std::ops::RangeInclusive;

use crate::{
    ray::Ray,
    vector::{Direction, Point},
};

pub enum ObjectGeometry {
    Sphere { center: Point, radius: f64 },
}

impl ObjectGeometry {
    pub fn hit(
        &self,
        ray: &Ray,
        ray_length_min_max: RangeInclusive<f64>,
    ) -> Option<(f64, Point, Direction)> {
        match self {
            Self::Sphere { center, radius } => {
                Self::hit_sphere(center, *radius, ray, ray_length_min_max)
            }
        }
    }

    fn hit_sphere(
        center: &Point,
        radius: f64,
        ray: &Ray,
        ray_length_min_max: RangeInclusive<f64>,
    ) -> Option<(f64, Point, Direction)> {
        let ray_to_center_direction = center.point_towards(*ray.origin());

        let a = ray.direction().length_squared();
        let half_b = ray_to_center_direction.dot(*ray.direction());
        let c = ray_to_center_direction.length_squared() - radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0. {
            None
        } else {
            let disc_sqrt = discriminant.sqrt();
            Some((-half_b - disc_sqrt) / a)
                .map(|root| {
                    if ray_length_min_max.contains(&root) {
                        root
                    } else {
                        (-half_b + disc_sqrt) / a
                    }
                })
                .filter(|root| ray_length_min_max.contains(root))
                .map(|root| {
                    let point_of_intersection = ray.at(root);
                    // Divide by radius instead of calling Direction::unit_vector here
                    // because the signal of `radius` is used
                    let normal = center.point_towards(point_of_intersection) / radius;
                    (root, point_of_intersection, normal)
                })
        }
    }
}
