use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HitTableList {
    pub list_size: u32,
    pub list: Vec<Box<dyn Hittable>>,
}

impl HitTableList {
    pub fn new() -> HitTableList {
        HitTableList {
            list_size: 0,
            list: vec![],
        }
    }

    pub fn add(&mut self, item: Box<dyn Hittable>) {
        self.list.push(item);
    }
}

impl Hittable for HitTableList {
    fn hit(
        &self,
        r: &Ray,
        t_min: f32,
        t_max: f32,
        rec: &mut HitRecord,
    ) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for item in self.list.iter() {
            if item.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                // TODO: Why cant I just assign rec = temp_rec?
                rec.normal = temp_rec.normal;
                rec.p = temp_rec.p;
                rec.t = closest_so_far;
            }
        }
        hit_anything
    }
}
