use crate::{
    intersection::Intersection,
    material::Material,
    medium::Medium,
    ray::Ray,
    acceleration::aabb::AABB,
    utils::Matrix4,
};
use std::sync::Arc;

mod sphere;
mod disk;
mod triangle;
mod rectangle;
mod cube;
mod mesh;

use sphere::Sphere;
use disk::Disk;
use triangle::Triangle;
use rectangle::Rectangle;
use cube::Cube;
use mesh::Mesh;

pub trait IntersectWithRay {
    fn intersect(&self, ray: &Ray, t_min: f64) -> Intersection;
}

pub struct Shape {
    pub flip_normal: bool,
    pub area: f64,
    pub aabb: AABB,
    pub material: Arc<dyn Material>,
    pub interior_medium: Arc<dyn Medium>,
    pub exterior_medium: Arc<dyn Medium>,
    pub to_world: Option<Matrix4>,
    pub to_object: Option<Matrix4>,
    pub inner: ShapeUnified,
}

impl Shape {
    pub fn sample(&self) -> Intersection {
        todo!()
    }
    pub fn area(&self) -> f64 {
        self.area
    }
    pub fn has_emission(&self) -> bool {
        self.material.has_emission()
    }
    pub fn aabb(&self) -> AABB {
        self.aabb.clone()
    }
}

impl IntersectWithRay for Shape {
    fn intersect(&self, ray: &Ray, t_min: f64) -> Intersection {
        match &self.inner {
            ShapeUnified::Sphere(sphere) => sphere.intersect(
                ray,
                t_min,
                self.material.clone(),
                self.interior_medium.clone(),
                self.exterior_medium.clone(),
                &self.to_world,
                &self.to_object,
                self.flip_normal,
            ),
            ShapeUnified::Disk(disk) => disk.intersect(ray, t_min),
            ShapeUnified::Triangle(triangle) => triangle.intersect(
                ray,
                t_min,
                self.material.clone(),
                self.interior_medium.clone(),
                self.exterior_medium.clone(),
                &self.to_world,
                &self.to_object,
                self.flip_normal,
            ),
            ShapeUnified::Rectangle(rectangle) => rectangle.intersect(ray, t_min),
            ShapeUnified::Cube(cube) => cube.intersect(ray, t_min),
            ShapeUnified::Mesh(mesh) => mesh.intersect(ray, t_min),
        }
    }
}

pub enum ShapeUnified {
    Sphere(Sphere),
    Disk(Disk),
    Triangle(Triangle),
    Rectangle(Rectangle),
    Cube(Cube),
    Mesh(Mesh),
}