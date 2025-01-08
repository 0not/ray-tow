use core::ops::Range;

use crate::material::Material;
use crate::ray::Ray;
use crate::Vec3;

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

#[cfg(feature = "std")]
impl<T> Hittable for std::vec::Vec<T>
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

impl<T> Hittable for [Option<T>; 488]
where
    T: Hittable + Sync,
{
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        let mut closest_so_far = interval.end;
        let mut hit_record = None;

        for hittable in self.iter() {
            match hittable {
                None => continue,
                Some(hittable) => {
                    if let Some(record) = hittable.hit(ray, interval.start..closest_so_far) {
                        closest_so_far = record.t;
                        hit_record = Some(record);
                    }
                }
            }
        }

        hit_record
    }
}
