use vec3::Vec3;
use rand::{thread_rng, Rng};

pub fn random_in_unit_sphere() -> Vec3 {
    // Initialize with random values, so that the length is >= 1
    let mut point = Vec3::new(1.0, 1.0, 1.0);
    let mut rng = thread_rng();

    while point.squared_length() >= 1.0 {
        point = Vec3 {
            x: rng.gen_range(-1.0, 1.0),
            y: rng.gen_range(-1.0, 1.0),
            z: rng.gen_range(-1.0, 1.0),
        };
    }

    point
}

pub fn random_in_unit_circle() -> Vec3 {
    // Initialize with random values, so that the length is >= 1
    let mut point = Vec3::new(1.0, 1.0, 0.0);
    let mut rng = thread_rng();

    while point.squared_length() >= 1.0 {
        point = Vec3 {
            x: rng.gen_range(-1.0, 1.0),
            y: rng.gen_range(-1.0, 1.0),
            z: 0.0,
        };
    }

    point
}

pub fn schlick(cosine: f64, reflection_index: f64) -> f64 {
    let r0 = (1.0 - reflection_index) / (1.0 + reflection_index);
    let r0_squared = r0 * r0;
    r0_squared + (1.0 - r0_squared) * (1.0 - cosine).powi(5)
}
