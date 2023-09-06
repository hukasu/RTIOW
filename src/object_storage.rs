use std::ops::RangeInclusive;

use crate::{
    object::{HitRecord, Object},
    ray::Ray,
};

pub trait ObjectStorage: IntoIterator<Item = Object> {
    fn add_object(&mut self, object: Object);
    fn clear(&mut self);
    fn find_intersection<'a>(
        &self,
        ray: &'a Ray,
        ray_length_min_max: RangeInclusive<f64>,
    ) -> Option<HitRecord<'a>>;
}

impl ObjectStorage for Vec<Object> {
    fn add_object(&mut self, object: Object) {
        self.push(object);
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn find_intersection<'a>(
        &self,
        ray: &'a Ray,
        ray_length_min_max: RangeInclusive<f64>,
    ) -> Option<HitRecord<'a>> {
        self.iter()
            .filter_map(|object| object.hit(ray, ray_length_min_max.clone()))
            .min_by(|hit_record_lhs, hit_record_rhs| {
                hit_record_lhs
                    .distance_from_ray
                    .total_cmp(&hit_record_rhs.distance_from_ray)
            })
    }
}
