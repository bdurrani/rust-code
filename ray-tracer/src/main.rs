use crate::vec3::Vec3;
use hittable::{HitRecord, Hittable};
use hittableList::HitTableList;
use sphere::Sphere;
use std::f32;

mod hittable;
mod hittableList;
mod ray;
mod sphere;
mod vec3;

use ray::Ray;

fn color(r: &Ray, world: &dyn Hittable) -> Vec3 {
    let mut rec = HitRecord::new();
    return if world.hit(&r, 0.0, f32::MAX, &mut rec) {
        0.5 * Vec3::new(
            rec.normal.x() + 1.0,
            rec.normal.y() + 1.0,
            rec.normal.z() + 1.0,
        )
    } else {
        let unit_direction: Vec3 = Vec3::unit_vector(r.direction());
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    };
}

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let lower_left_corner: Vec3 = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let mut list = HitTableList::new();
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    list.add(Box::new(sphere));
    let sphere = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    list.add(Box::new(sphere));
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u: f32 = i as f32 / nx as f32;
            let v: f32 = j as f32 / ny as f32;
            let b: f32 = 0.2;
            let r: Ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical,
            );
            let p = r.point_at_parameter(2.0);
            let col: Vec3 = color(&r, &list);
            let ir: i32 = (255.99 * col[0]) as i32;
            let ig: i32 = (255.99 * col[1]) as i32;
            let ib: i32 = (255.99 * col[2]) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
