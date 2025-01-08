pub mod sphere;

use core::ops::Range;

use crate::hit_record::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::shapes::sphere::Sphere;

#[non_exhaustive]
pub enum Shape {
    Sphere(Sphere),
}

impl Hittable for Shape {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        match self {
            Shape::Sphere(sphere) => sphere.hit(ray, interval),
        }
    }
}
