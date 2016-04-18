use vec3::Vec3;
use ray::Ray;
use std::vec::Vec;
use material::Material;
use std::rc::Rc;
use sphere::Sphere;

pub struct Intersection {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Rc<Material>,
}

impl Intersection {
    pub fn new(t: f64, point: Vec3, normal: Vec3, material: Rc<Material>) -> Intersection {
        Intersection {
            t: t,
            point: point,
            normal: normal,
            material: material,
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;
}

impl Hitable for Vec<Sphere> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let mut result: Option<Intersection> = None;
        let mut closets_so_far = t_max;

        for e in self {
            match e.hit(ray, t_min, closets_so_far) {
                Some(record) => {
                    closets_so_far = record.t;
                    result = Some(record);
                }
                None => {}
            }
        }

        result
    }
}
