use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::Vec3;

pub struct Sphere {
    radius: f32,
    center: Vec3,
}

impl Sphere {
    pub fn new(cen: Vec3, r: f32) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - *center;
        let a = Vec3::dot(&r.direction(), &r.direction());
        let b = 2.0 * Vec3::dot(&oc, &r.direction());
        let c = Vec3::dot(&oc, &oc) - radius * radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - f32::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - center) / self.radius;
                return true;
            }
            let temp = (-b + f32::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - center) / self.radius;
                return true;
            }
        }
        return false;
    }
}
