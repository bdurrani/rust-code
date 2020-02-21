use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord::new_from(
            0.0,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
        )
    }

    pub fn new_from(t: f32, p: Vec3, normal: Vec3) -> HitRecord {
        HitRecord { t, p, normal }
    }
}

pub trait Hittable {
    fn hit(
        &self,
        r: &Ray,
        t_min: f32,
        t_max: f32,
        hit_records: &mut HitRecord,
    ) -> bool;
}
