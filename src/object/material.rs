use crate::{
    ray::Ray,
    vector::{Colour, Direction},
};

use super::{HitRecord, RayScatter};

#[derive(Debug, Clone, Copy)]
pub enum ObjectMaterial {
    Lambert { albedo: Colour },
    Metal { albedo: Colour, fuzzy_scatter: f64 },
    Dialectric { refraction_index: f64 },
}

impl ObjectMaterial {
    pub fn scatter(&self, hit_record: &HitRecord) -> RayScatter {
        match self {
            ObjectMaterial::Lambert { albedo } => Self::lambert_scatter(hit_record, albedo),
            ObjectMaterial::Metal {
                albedo,
                fuzzy_scatter,
            } => Self::metal_scatter(hit_record, albedo, *fuzzy_scatter),
            ObjectMaterial::Dialectric { refraction_index } => {
                Self::dialectric_scatter(hit_record, *refraction_index)
            }
        }
    }

    fn lambert_scatter(hit_record: &HitRecord, albedo: &Colour) -> RayScatter {
        let scatter_direction = hit_record.normal
            + hit_record
                .normal
                .random_direction_in_hemisphere()
                .unit_vector();
        RayScatter {
            attenuation: *albedo,
            scattered: Ray::new(
                hit_record.point_of_intersection,
                if scatter_direction.is_zero() {
                    hit_record.normal
                } else {
                    scatter_direction
                },
            ),
        }
    }

    fn metal_scatter(hit_record: &HitRecord, albedo: &Colour, fuzzy_scatter: f64) -> RayScatter {
        let reflect = hit_record
            .intersecting_ray
            .direction()
            .reflect(&hit_record.normal)
            + fuzzy_scatter * Direction::new_random_in_unit_sphere().unit_vector();
        RayScatter {
            attenuation: *albedo,
            scattered: Ray::new(hit_record.point_of_intersection, reflect),
        }
    }

    fn dialectric_scatter(hit_record: &HitRecord, refraction_index: f64) -> RayScatter {
        let refraction_ratio = if hit_record.front_face {
            1. / refraction_index
        } else {
            refraction_index
        };
        let unit_direction = hit_record.intersecting_ray.direction().unit_vector();

        let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let reflectance = {
            let r0 = ((1. - refraction_ratio) / (1. + refraction_ratio)).powi(2);
            r0 + (1. - r0) * (1. - cos_theta).powi(5)
        };

        let direction = if cannot_refract || reflectance > rand::random() {
            unit_direction.reflect(&hit_record.normal)
        } else {
            unit_direction.refract(&hit_record.normal, refraction_ratio)
        };

        RayScatter {
            attenuation: Colour::new(1., 1., 1.),
            scattered: Ray::new(hit_record.point_of_intersection, direction),
        }
    }
}
