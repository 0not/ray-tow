use std::ops::Range;

use crate::material::Material;
use crate::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[derive(Default, PartialEq, Eq)]
pub enum FaceSide {
    #[default]
    Front,
    Back,
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub face_side: FaceSide,
    pub material: Material,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Vec3::ZERO,
            normal: Vec3::ZERO,
            t: 0.,
            face_side: FaceSide::default(),
            material: Material::Lambertian { albedo: Vec3::ONE },
        }
    }
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f64, face_side: FaceSide, material: Material) -> Self {
        Self {
            point,
            normal,
            t,
            face_side,
            material,
        }
    }

    // outward_normal should have unit length
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.face_side = if ray.direction.dot(outward_normal) < 0. {
            FaceSide::Front
        } else {
            FaceSide::Back
        };

        self.normal = match self.face_side {
            FaceSide::Front => outward_normal,
            FaceSide::Back => -outward_normal,
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord>;
}

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

impl<T> Hittable for Vec<T>
where
    T: Hittable + Sync,
{
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        let mut closest_so_far = interval.end;
        let mut hit_record = None;

        for hittable in self {
            if let Some(record) = hittable.hit(ray, interval.start..closest_so_far) {
                closest_so_far = record.t;
                hit_record = Some(record);
            }
        }

        hit_record
    }
}
