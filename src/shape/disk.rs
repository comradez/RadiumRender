use std::f64::consts::PI;
use crate::{utils::Vector3, ray::Ray, intersection::Intersection};

use super::IntersectWithRay;

pub struct Disk {

}

impl IntersectWithRay for Disk {
    fn intersect(&self, ray: &Ray, t_min: f64) -> Option<Intersection> {
        todo!()
    }
}