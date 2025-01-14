use core::ops::Range;

use crate::utils::{abs, sqrt};
use crate::{Vec2, Vec3};
use rand::distributions::Standard;
use rand::Rng;
use rand_distr::{Distribution, UnitDisc, UnitSphere};

pub fn refract(uv: Vec3, normal: Vec3, n1_over_n2: f64) -> Vec3 {
    let cos_theta = (-uv).dot(normal).min(1.);
    let r_out_perp = n1_over_n2 * (uv + cos_theta * normal);
    let r_out_parallel = -sqrt(abs(1.0 - r_out_perp.length_squared())) * normal;
    r_out_perp + r_out_parallel
}

pub fn sample_square(rng: &mut impl Rng) -> Vec2 {
    let vec: Vec2 = rng.sample(Standard);
    vec - 0.5
}

pub fn random_unit_vector(rng: &mut impl Rng) -> Vec3 {
    let unit_sphere = UnitSphere.sample(rng);
    Vec3::from_array(unit_sphere)
}

pub fn random_unit_hemisphere(normal: Vec3, rng: &mut impl Rng) -> Vec3 {
    let vec = random_unit_vector(rng);
    if vec.dot(normal) > 0. {
        vec
    } else {
        -vec
    }
}

pub fn random_in_unit_disc(rng: &mut impl Rng) -> Vec2 {
    let unit_disc = UnitDisc.sample(rng);
    Vec2::from_array(unit_disc)
}

pub fn random_in_range(range: Range<f64>, rng: &mut impl Rng) -> Vec3 {
    // let mut rng = thread_rng();

    // let arr: [f64; 3] = rng.gen();
    // Vec3::from_array(rng.gen())

    // let uniform: [f64; 3] = Uniform::new(min, max).sample(&mut rng);
    // rng.sample::<Vec3, Standard>(Standard) * (max - min) + min
    rng.gen::<Vec3>() * (range.end - range.start) + range.start

    // Vec3::new(
    //     rng.gen_range(range.clone()),
    //     rng.gen_range(range.clone()),
    //     rng.gen_range(range),
    // )
}
