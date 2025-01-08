#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod camera;
pub mod hit_record;
pub mod material;
pub mod raw_image_buffer;
pub mod ray;
pub mod shapes;
pub mod utils;
pub mod vectors;

// pub use camera::Camera;
// pub use material::Material;
// pub use raw_image_buffer::RawImageBuffer;
// pub use ray::{HitRecord, Hittable, Ray, Shape, Sphere};

pub type Color = glam::DVec3;
pub type Vec3 = glam::DVec3;
pub type Vec2 = glam::DVec2;
