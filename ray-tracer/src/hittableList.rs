use crate::hittable::{HitRecord, Hittable};

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

    pub fn add(&self, item: Box<dyn Hittable>) {
        self.add(item);
    }
}
