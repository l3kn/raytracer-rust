use vec3::Vec3;
use ray::Ray;
use helper;
use hitable::Intersection;

use rand::{thread_rng, Rng};

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &Intersection) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    pub albedo: Vec3, // Fraction of light that is reflected
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &Intersection) -> Option<(Ray, Vec3)> {
        let target = record.point + record.normal + helper::random_in_unit_sphere();
        let scattered = Ray {
            origin: record.point,
            direction: target - record.point,
        };
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &Intersection) -> Option<(Ray, Vec3)> {
        let reflected = Vec3::reflect(ray.direction.unit_vector(), record.normal);
        let scattered = Ray {
            origin: record.point,
            direction: reflected + self.fuzz * helper::random_in_unit_sphere(),
        };
        if Vec3::dot(scattered.direction, record.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub reflection_index: f64,
    pub albedo: Vec3,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &Intersection) -> Option<(Ray, Vec3)> {
        let dot_direction = Vec3::dot(ray.direction, record.normal) / ray.direction.length();

        let (outward_normal, ni_over_nt, cosine) = if Vec3::dot(ray.direction, record.normal) >
                                                      0.0 {
            let cos = self.reflection_index * dot_direction;
            (-record.normal, self.reflection_index, cos)
        } else {
            let cos = -dot_direction;
            (record.normal, 1.0 / self.reflection_index, cos)
        };

        let reflected = Vec3::reflect(ray.direction.unit_vector(), record.normal);

        match Vec3::refract(ray.direction, outward_normal, ni_over_nt) {
            Some(refracted) => {
                let mut rng = thread_rng();
                if rng.gen_range(0.0, 1.0) < helper::schlick(cosine, self.reflection_index) {
                    Some((Ray::new(record.point, reflected), self.albedo))
                } else {
                    Some((Ray::new(record.point, refracted), self.albedo))
                }
            }
            None => Some((Ray::new(record.point, reflected), self.albedo)),
        }
    }
}
