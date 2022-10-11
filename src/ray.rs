use std::sync::Arc;

use crate::medium::Medium;
use crate::utils::Vector3;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
    pub medium: Option<Arc<dyn Medium>>,
}

impl Ray {
    pub fn new(origin: &Vector3, direction: &Vector3) -> Self {
        Self {
            origin: origin.clone(),
            direction: direction.normalize(),
            medium: None,
        }
    }

    pub fn point_at_param(&self, t: f64) -> Vector3 {
        self.origin + t * self.direction
    }
}