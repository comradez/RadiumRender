use std::f64::consts::PI;
use crate::{utils::Vector3, ray::Ray, intersection::Intersection};

use super::IntersectWithRay;

pub struct Rectangle {

}

impl IntersectWithRay for Rectangle {
    fn intersect(&self, ray: &Ray, t_min: f64) -> Intersection {
        todo!()
    }
}