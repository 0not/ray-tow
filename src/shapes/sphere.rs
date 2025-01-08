use core::ops::Range;

use crate::hit_record::{FaceSide, HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (half_b - sqrtd) / a;
        // Check first root
        if !interval.contains(&root) {
            // Check second root
            root = (half_b + sqrtd) / a;
            if !interval.contains(&root) {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;

        let mut hit_record = HitRecord {
            point,
            normal: outward_normal,
            t: root,
            face_side: FaceSide::Front,
            material: self.material.clone(),
        };

        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }
}
