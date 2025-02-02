use indicatif::ParallelProgressIterator;
use itertools::iproduct;
use rand::thread_rng;
use rayon::prelude::*;

use crate::hit_record::Hittable;
use crate::raw_image_buffer::RawImageBuffer;
use crate::ray::Ray;
use crate::vectors::{random_in_unit_disc, sample_square};
use crate::{Color, Vec3};

#[derive(Default, Debug)]
pub struct Camera {
    pub position: Vec3,
    // pub direction: Vec3,
    // pub up: Vec3,
    // pub right: Vec3,
    // pub fov: f64,
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    // pub near: f64,
    // pub far: f64,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    // defocus_angle: f64,
    f_stop: Option<f64>,
}

impl Camera {
    pub fn init() -> CameraBuilder {
        CameraBuilder::default()
    }

    pub fn render<T>(&self, world: &T) -> RawImageBuffer
    where
        T: Hittable + std::marker::Sync,
    {
        // Create a progress bar for looping over pixels
        // let pb = ProgressBar::new((self.image_width * self.image_height) as u64);

        let mut rawbuf = RawImageBuffer::new(self.image_width, self.image_height);

        // for y in 0..self.image_height {
        //     for x in 0..self.image_width {
        //         let mut pixel_color = Color::ZERO;

        //         for _sample_n in 0..self.samples_per_pixel {
        //             let ray = self.create_ray(x, y);
        //             pixel_color += Camera::ray_color(&ray, self.max_depth, world);
        //         }

        //         rawbuf.push_color(pixel_color / self.samples_per_pixel as f64);

        //         pb.inc(1);
        //     }
        // }
        let xys: Vec<_> = iproduct!(0..self.image_height, 0..self.image_width).collect();
        let colors: Vec<Color> = xys
            .par_iter()
            .progress_count(xys.len() as u64)
            .map(|(y, x)| {
                let mut pixel_color = Color::ZERO;

                for _sample_n in 0..self.samples_per_pixel {
                    let ray = self.create_ray(*x, *y);
                    pixel_color += Camera::ray_color(&ray, self.max_depth, world);
                }

                pixel_color / self.samples_per_pixel as f64
            })
            .collect();

        colors
            .into_iter()
            .for_each(|color| rawbuf.push_color(color));
        // pb.inc(1);
        // Finish progress bar
        // pb.finish();

        // println!(
        //     "Rendered {}x{} pixels in {:?}",
        //     self.image_width,
        //     self.image_height,
        //     pb.elapsed()
        // );

        rawbuf
    }

    fn ray_color<T>(ray: &Ray, depth: u32, world: &T) -> Color
    where
        T: Hittable + std::marker::Sync,
    {
        if depth == 0 {
            return Color::ZERO;
        }

        if let Some(hit_record) = world.hit(ray, 0.001..f64::INFINITY) {
            if let Some((scattered_ray, attenuation)) =
                hit_record.material.scatter(ray, &hit_record)
            {
                attenuation * Camera::ray_color(&scattered_ray, depth - 1, world)
            } else {
                Color::ZERO
            }
            // let ray = Ray::new(hit_record.point, direction);
            // 0.5 * Camera::ray_color(&ray, depth - 1, world)
        } else {
            let unit_direction = ray.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Color::ONE + t * Color::new(0.5, 0.7, 1.0)
        }
    }

    fn create_ray(&self, x: u32, y: u32) -> Ray {
        let mut rng = thread_rng();
        let offset = sample_square(&mut rng);

        let pixel_sample = self.pixel00_loc
            + (x as f64 + offset.x) * self.pixel_delta_u
            + (y as f64 + offset.y) * self.pixel_delta_v;

        let origin = if self.f_stop.is_none() {
            self.position
        } else {
            let defocus = random_in_unit_disc(&mut rng);
            self.position + defocus.x * self.defocus_disk_u + defocus.y * self.defocus_disk_v
        };
        let direction = pixel_sample - origin;
        Ray::new(origin, direction)
    }
}

pub struct CameraBuilder {
    image_width: u32,
    aspect_ratio: f64,
    position: Vec3,
    look_at: Vec3,
    up: Vec3,
    samples_per_pixel: u32,
    max_depth: u32,
    // vfov: f64, // vertical field of view, in degrees
    // defocus_angle: f64,
    /// Focal length of lens
    focal_length: f64,
    /// f-stop of lens (aperture size)
    f_stop: Option<f64>,
    sensor_width: f64,
    sensor_height: f64,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            image_width: 400,
            aspect_ratio: 16.0 / 9.0,
            position: Vec3::ZERO,
            look_at: Vec3::new(0., 0., -1.),
            up: Vec3::new(0., 1., 0.),
            samples_per_pixel: 1,
            max_depth: 10,
            // vfov: 90.,
            // defocus_angle: 0.,
            focal_length: 1.,
            f_stop: None,
            sensor_width: 36e-3,
            sensor_height: 24e-3,
        }
    }
}
impl CameraBuilder {
    pub fn image_width(mut self, image_width: u32) -> Self {
        self.image_width = image_width;
        self
    }

    // pub fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
    //     self.aspect_ratio = aspect_ratio;
    //     self
    // }

    pub fn position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn look_at(mut self, look_at: Vec3) -> Self {
        self.look_at = look_at;
        self
    }

    pub fn up(mut self, up: Vec3) -> Self {
        self.up = up;
        self
    }

    pub fn focal_length(mut self, focal_length: f64) -> Self {
        self.focal_length = focal_length;
        self
    }

    pub fn samples_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }

    // pub fn vfov(mut self, vfov: f64) -> Self {
    //     self.vfov = vfov;
    //     self
    // }

    // pub fn defocus_angle(mut self, defocus_angle: f64) -> Self {
    //     self.defocus_angle = defocus_angle;
    //     self
    // }

    pub fn f_stop(mut self, f_stop: f64) -> Self {
        self.f_stop = Some(f_stop);
        self
    }

    pub fn sensor_dimensions(mut self, width: f64, height: f64) -> Self {
        self.aspect_ratio = width / height;
        self.sensor_width = width;
        self.sensor_height = height;
        self
    }

    pub fn build(self) -> Camera {
        // Calculate height
        let image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        // let actual_aspect_ratio = self.image_width as f64 / image_height as f64;

        // Setup camera
        let focal_dist = self.position.distance(self.look_at);
        // let focal_length = 1.;
        // let theta = self.vfov.to_radians();
        // let h = self.focal_length * (theta / 2.).tan();
        // let viewport_height = 2. * h;
        // let viewport_width = viewport_height * actual_aspect_ratio;
        // let viewport_height = 24e-3;
        // let viewport_width = 36e-3;
        let viewport_height = self.sensor_height * focal_dist / self.focal_length;
        let viewport_width = self.sensor_width * focal_dist / self.focal_length;

        // Calculate camera basis vectors
        let w = (self.position - self.look_at).normalize();
        let u = self.up.cross(w).normalize();
        let v = w.cross(u);

        // Calculate viewport basis (horizontal and vertical)
        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        // Calculate the horizontal and vertical delta vectors (pixel to pixel)
        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Upper left pixel
        let viewport_upper_left =
            self.position - focal_dist * w - viewport_u / 2. - viewport_v / 2.;
        // let mag = viewport_upper_left.length();
        // let viewport_upper_left = viewport_upper_left - 0.01 * viewport_upper_left;
        // let viewport_upper_left = viewport_upper_left.normalize() * focal_dist;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the defocus disk basis vectors
        // let aperture_radius = self.focal_length * (self.defocus_angle / 2.).to_radians().tan();
        let aperture_radius = if self.f_stop.is_some() {
            self.focal_length / (2. * self.f_stop.unwrap())
        } else {
            0.
        };
        let defocus_disk_u = aperture_radius * u;
        let defocus_disk_v = aperture_radius * v;

        Camera {
            position: self.position,
            // pub direction: Vec3,
            // pub up: Vec3,
            // pub right: Vec3,
            // pub fov: f64,
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            image_height,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
            // pub near: f64,
            // pub far: f64,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
            // defocus_angle: self.defocus_angle,
            f_stop: self.f_stop,
        }
    }
}
