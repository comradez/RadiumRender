use std::path::Path;
use std::sync::Arc;

use anyhow::Result;

use crate::camera::Camera;
use crate::integrator::Integrator;
use crate::material::Material;
use crate::shape::Shape;
use crate::media::Media;
use crate::env_map::EnvMap;

pub struct Renderer {
    camera: Arc<Camera>,
    integrator: Arc<Integrator>,
    env_map: Option<Arc<EnvMap>>,
    shapes: Vec<Arc<dyn Shape>>,
    materials: Vec<Arc<dyn Material>>,
    medias: Vec<Arc<dyn Media>>,
}

impl Renderer {
    pub fn render(&self, _output: &Path) -> Result<()> {
        todo!()
    }
}