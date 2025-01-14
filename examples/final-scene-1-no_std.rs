#![no_std]
#![no_main]

extern crate panic_halt;

use itertools::iproduct;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use ray_tow::camera::Camera;
use ray_tow::material::Material;
use ray_tow::shapes::{sphere::Sphere, Shape};
use ray_tow::vectors::{random_in_range, random_unit_vector};
use ray_tow::Vec3;

#[no_mangle]
fn main() {
    // Setup world
    // Use a seed for reproducibility
    let seed = 42;
    let mut rng = SmallRng::seed_from_u64(seed);
    // 488 = 1 + 22 * 22 + 3
    let mut world: [Option<Shape>; 488] = [const { None }; 488];
    let mat_ground = Material::Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    };

    world[0] = Some(Shape::Sphere(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        mat_ground,
    )));

    for (n, (a, b)) in iproduct!(-11..11, -11..11).enumerate() {
        let choose_mat: f64 = rng.gen();
        let center = Vec3::new(
            a as f64 + 0.9 * rng.gen::<f64>(),
            0.2,
            b as f64 + 0.9 * rng.gen::<f64>(),
        );

        let shape = if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
            let mat = if choose_mat < 0.8 {
                // diffuse
                let albedo = random_unit_vector(&mut rng) * random_unit_vector(&mut rng);
                Material::Lambertian { albedo }
            } else if choose_mat < 0.95 {
                // metal
                let albedo = random_in_range(0.5..1., &mut rng);
                let fuzz = rng.gen_range(0.0..0.5);
                Material::Metal { albedo, fuzz }
            } else {
                // glass
                Material::Dielectric {
                    index_of_refraction: 1.5,
                }
            };

            Some(Shape::Sphere(Sphere::new(center, 0.2, mat)))
        } else {
            None
        };

        world[n + 1] = shape;
    }

    let sphere_1 = Shape::Sphere(Sphere::new(
        Vec3::new(0., 1., 0.),
        1.,
        Material::Dielectric {
            index_of_refraction: 1.5,
        },
    ));
    let sphere_2 = Shape::Sphere(Sphere::new(
        Vec3::new(-4., 1., 0.),
        1.,
        Material::Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        },
    ));
    let sphere_3 = Shape::Sphere(Sphere::new(
        Vec3::new(4., 1., 0.),
        1.,
        Material::Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.,
        },
    ));

    world[485] = Some(sphere_1);
    world[486] = Some(sphere_2);
    world[487] = Some(sphere_3);

    let camera = Camera::init()
        .position(Vec3::new(13., 2., 3.))
        .look_at(Vec3::new(0., 0., 0.))
        .up(Vec3::Y)
        // .aspect_ratio(36e-3 / 24e-3)
        .sensor_dimensions(36e-3, 24e-3)
        .image_width(400)
        .samples_per_pixel(50)
        .max_depth(10)
        // .image_width(1200)
        // .samples_per_pixel(500)
        // .max_depth(50)
        // .vfov(20.)
        .f_stop(0.6)
        .focal_length(60e-3)
        .build();

    camera.with_world(&world).for_each(|_pixel| {});
}
