use itertools::iproduct;
use rand::{Rng, SeedableRng};
use ray_tow::material::Material;
use ray_tow::vectors::{random_in_range, random_unit_vector};
use ray_tow::{camera::Camera, ray::Shape, ray::Sphere, Vec3};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup world
    // Use a seed for reproducibility
    let seed = 42;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let mut world: Vec<Shape> = vec![];
    let mat_ground = Material::Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    };
    world.push(Shape::Sphere(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        mat_ground,
    )));

    let shapes = iproduct!(-11..11, -11..11).flat_map(|(a, b)| {
        let choose_mat: f64 = rng.gen();
        let center = Vec3::new(
            a as f64 + 0.9 * rng.gen::<f64>(),
            0.2,
            b as f64 + 0.9 * rng.gen::<f64>(),
        );

        if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
            let mat = if choose_mat < 0.8 {
                // diffuse
                let albedo = random_unit_vector() * random_unit_vector();
                Material::Lambertian { albedo }
            } else if choose_mat < 0.95 {
                // metal
                let albedo = random_in_range(0.5..1.);
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
        }
    });

    world.extend(shapes);

    world.push(Shape::Sphere(Sphere::new(
        Vec3::new(0., 1., 0.),
        1.,
        Material::Dielectric {
            index_of_refraction: 1.5,
        },
    )));
    world.push(Shape::Sphere(Sphere::new(
        Vec3::new(-4., 1., 0.),
        1.,
        Material::Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        },
    )));
    world.push(Shape::Sphere(Sphere::new(
        Vec3::new(4., 1., 0.),
        1.,
        Material::Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.,
        },
    )));

    let camera = Camera::init()
        .position(Vec3::new(13., 2., 3.))
        .look_at(Vec3::new(0., 0., 0.))
        .up(Vec3::new(0., 1., 0.))
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

    println!("{:?}", camera);
    let render_buffer = camera.render(&world);

    // Get timestamp for keeping a record of the ray tracer progress
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or(std::time::Duration::from_secs(0))
        .as_secs();

    render_buffer.save(format!("output/render-{timestamp}.png"))?;
    render_buffer.save("output/latest.png")?;

    Ok(())
}
