use std::path::Path;
use std::sync::Arc;

use anyhow::Result;

use crate::camera::Camera;
use crate::integrator::Integrator;
use crate::material::Material;
use crate::shape::Shape;
use crate::medium::Medium;
use crate::env_map::EnvMap;

pub struct Renderer {
    camera: Arc<Camera>,
    integrator: Arc<Integrator>,
    env_map: Option<Arc<EnvMap>>,
    shapes: Vec<Arc<Shape>>,
    materials: Vec<Arc<dyn Material>>,
    medias: Vec<Arc<dyn Medium>>,
}

impl Renderer {
    pub fn render(&self, _output: &Path) -> Result<()> {
        todo!()
    }
}