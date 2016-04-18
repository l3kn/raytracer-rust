use vec3::Vec3;
use ray::Ray;
use hitable::{Hitable, Intersection};
use material::Material;
use std::rc::Rc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<Material>,
    pub radius2: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<Material>) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material,
            radius2: radius * radius,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {

        let oc = ray.origin - self.center;
        let dir_length = ray.direction.squared_length();

        let dot_dir_oc = Vec3::dot(ray.direction, oc);
        let discriminant = dot_dir_oc.powi(2) - (oc.squared_length() - self.radius2) * dir_length;
        let discriminant_sqrt = discriminant.sqrt();

        if discriminant < 0.0 {
            return None;
        } else {
            let mut tmp = (-dot_dir_oc - discriminant_sqrt) / dir_length;
            if t_min < tmp && tmp < t_max {
                let point = ray.point_at_parameter(tmp);
                let normal = (point - self.center).unit_vector();
                return Some(Intersection::new(tmp, point, normal, self.material.clone()));
            }

            tmp = (-dot_dir_oc + discriminant_sqrt) / dir_length;
            if t_min < tmp && tmp < t_max {
                let point = ray.point_at_parameter(tmp);
                let normal = (point - self.center).unit_vector();
                return Some(Intersection::new(tmp, point, normal, self.material.clone()));
            }
            return None;
        }
    }
}
// condition:
//     len(orig + t*dir - center) = r^2
// <=> dot(orig + t*dir - center, orig + t*dir - center) = r^2
// <=> dot(t*dir + (orig - center), t*dir + (orig - center) = r^2
// <=> t^2 dot(dir, dir) + 2t dot(dir, (orig - center)) + dot(orig - center, orig - center) = r^2
// <=> ...
// <=> t^2 + pt + q = 0
// where
//   oc = orig - center, p = 2 dot(dir, oc) / dot(dir, dir), q = (dot(oc, oc) - r^2) / dot(dir, dir)
//
// has solutions
// <=> discriminant >= 0 <=> (p/2)^2 - q >= 0
// <=> dot(dir, oc)^2 / dot(dir, dir)^2 - (dot(oc, oc) - r^2) / dot(dir, dir) >= 0
// <=> (because the dot-product is positive semidefinite)
//     dot(dir, oc)^2 - (dot(oc, oc) - r^2) * dot(dir, dir) >= 0
//
// solutions = -p/2 -+ discriminant.sqrt = dot(dir, oc) / dot(dir, dir) -+ (discriminant.sqrt / dot(dir, dir))
// note: the second / dot(dir, dir) is necessary, because we simplified the discriminant equation before
//
