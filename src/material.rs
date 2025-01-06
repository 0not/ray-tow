use crate::hit_record::{FaceSide, HitRecord};
use crate::ray::Ray;
use crate::vectors::{random_unit_vector, refract};
use crate::{Color, Vec3};
use rand::thread_rng;

#[non_exhaustive]
#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { index_of_refraction: f64 },
}

// TODO: Should this be a trait?
impl Material {
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let mut rng = thread_rng();
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = hit_record.normal + random_unit_vector(&mut rng);

                // Don't scatter near zero
                if scatter_direction.abs_diff_eq(Vec3::ZERO, 1e-8) {
                    scatter_direction = hit_record.normal;
                }

                let scattered_ray = Ray::new(hit_record.point, scatter_direction);
                let attenuation = *albedo;
                Some((scattered_ray, attenuation))
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = ray.direction.reflect(hit_record.normal).normalize();
                let scattered = Ray::new(
                    hit_record.point,
                    reflected + *fuzz * random_unit_vector(&mut rng),
                );
                let attenuation = *albedo;
                if scattered.direction.dot(hit_record.normal) > 0. {
                    Some((scattered, attenuation))
                } else {
                    None
                }
            }
            Material::Dielectric {
                index_of_refraction,
            } => {
                // TODO: Support other external materials besides air
                let refraction_ratio = if hit_record.face_side == FaceSide::Front {
                    1.0 / *index_of_refraction
                } else {
                    *index_of_refraction
                };

                let unit_direction = ray.direction.normalize();
                // let refracted = refract(unit_direction, hit_record.normal, refraction_ratio);

                // Account for total internal reflection
                let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                // Cannot refract
                let direction = if refraction_ratio * sin_theta > 1.0
                    || reflectance(cos_theta, refraction_ratio)
                        > random_unit_vector(&mut rng).x.abs()
                {
                    unit_direction.reflect(hit_record.normal)
                } else {
                    // Can refract
                    refract(unit_direction, hit_record.normal, refraction_ratio)
                };

                let scattered = Ray::new(hit_record.point, direction);
                let attenuation = Color::ONE;
                Some((scattered, attenuation))
            }
        }
    }
}

fn reflectance(cosine: f64, index_of_refraction: f64) -> f64 {
    // Use Schlick's approximation for reflectance
    let r0 = ((1. - index_of_refraction) / (1. + index_of_refraction)).powi(2);
    r0 + (1. - r0) * (1. - cosine).powi(5)
}
