use vec3::Vec3;
use ray::Ray;
use std::f64::consts::PI;
use helper::random_in_unit_circle;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3,
               look_at: Vec3,
               up: Vec3,
               vertical_fov: f64,
               aspect_ratio: f64,
               aperture: f64,
               focus_distance: f64)
               -> Camera {
        let theta = vertical_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let w = (look_from - look_at).unit_vector();
        let u = Vec3::cross(up, w).unit_vector();
        let v = Vec3::cross(w, u);

        Camera {
            origin: look_from,
            lower_left_corner: look_from - half_width * focus_distance * u -
                               half_height * focus_distance * v -
                               focus_distance * w,
            horizontal: 2.0 * half_width * focus_distance * u,
            vertical: 2.0 * half_height * focus_distance * v,
            lens_radius: aperture / 2.0,
            u: u,
            v: v,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_circle();
        let offset = rd.x * self.u + rd.y * self.v;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical -
                       self.origin - offset,
        }
    }
}
