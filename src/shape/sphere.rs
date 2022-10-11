use std::f64::consts::PI;
use vecmat::traits::Dot;

use crate::{utils::Vector3, ray::Ray, intersection::Intersection};

use super::IntersectWithRay;

pub struct Sphere {
    center: Vector3,
    radius: f64,
    inv_surface_area: f64,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64) -> Self {
        Self {
            center,
            radius,
            inv_surface_area: 1. / (4. * PI * radius * radius),
        }
    }
}

impl IntersectWithRay for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f64) -> Option<Intersection> {
        let v1 = ray.direction;
        let v2 = ray.origin - self.center;
        let a = v1.square_length();
        let b = 2. * v1.dot(v2);
        let c = v2.square_length() - self.radius * self.radius;

        let delta = b * b - 4. * a * c;
        if delta < 0. {
            None
        } else {
            let t1 = (- b - delta.sqrt()) / (2. * a);
            let t2 = (- b + delta.sqrt()) / (2. * a);
            if t1 >= t_min {
                let normal = (ray.point_at_param(t1) - self.center).normalize();
                todo!("Some Intersection at t1")
            } else if t2 >= t_min {
                let normal = (ray.point_at_param(t2) - self.center).normalize();
                todo!("Some Intersection at t2")
            } else {
                None
            }
        }
    }
}
