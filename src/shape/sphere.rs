use std::{f64::consts::PI, sync::Arc};
use vecmat::traits::Dot;

use crate::{
    utils::{Vector3, Matrix4, transform_direction, transform_point, uniform_sphere},
    ray::Ray,
    intersection::{Intersection, SurfaceIntersection},
    material::Material,
    medium::Medium,
};

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

    pub fn intersect(
        &self,
        ray: &Ray,
        t_min: f64,
        material: Arc<dyn Material>,
        interior_medium: Arc<dyn Medium>,
        exterior_medium: Arc<dyn Medium>,
        to_world: &Option<Matrix4>,
        to_object: &Option<Matrix4>,
        flip_normals: bool,
    ) -> Intersection {
        let (v1, v2) = if let Some(to_object) = to_object {
            (
                transform_direction(to_object, &ray.direction),
                transform_point(to_object, &ray.origin) - self.center
            )
        } else {
            (
                ray.direction,
                ray.origin - self.center
            )
        };
        let a = v1.square_length();
        let b = 2. * v1.dot(v2);
        let c = v2.square_length() - self.radius * self.radius;

        let delta = b * b - 4. * a * c;
        if delta >= 0. {
            let t1 = (- b - delta.sqrt()) / (2. * a);
            let t2 = (- b + delta.sqrt()) / (2. * a);
            let t = if t1 >= t_min { t1 } else { t2 };
            if t < t_min {
                return Intersection::None;
            }
            let position = ray.point_at_param(t);
            let normal = (position - self.center).normalize();
            let mut inside = false;
            if material.texture_mapping() {
                todo!("Texture Mapping....... TBD");
            }
            let (position, mut normal) = if let Some(to_world) = to_world {
                (transform_point(to_world, &position), transform_direction(to_world, &normal))
            } else {
                (position, normal)
            };
            let distance = (position - ray.origin).length();
            if !material.two_sided() && (flip_normals && c > 0. || !flip_normals && c < 0.) {
                return Intersection::AbsorbSurface(distance)
            }
            
            if c < 0. {
                normal = -normal;
                inside = !inside;
            }
            if flip_normals {
                normal = -normal;
                inside = !inside;
            }
            Intersection::Surface(SurfaceIntersection {
                position,
                normal,
                texcoord: None,
                inside,
                distance,
                material,
                interior_medium,
                exterior_medium,
            })
        } else {
            Intersection::None
        }
    }

    pub fn sample(
        &self,
        to_world: &Option<Matrix4>,
        material: Arc<dyn Material>,
        interior_medium: Arc<dyn Medium>,
        exterior_medium: Arc<dyn Medium>,
    ) -> Intersection {
        let normal = uniform_sphere();
        let position = self.center + self.radius * normal;

        let (position, normal) = if let Some(to_world) = to_world {
            (transform_point(to_world, &position), transform_direction(to_world, &normal))
        } else {
            (position, normal)
        };
        Intersection::Surface(SurfaceIntersection {
            position,
            normal,
            texcoord: None,
            inside: false,
            distance: f64::MAX,
            material,
            interior_medium,
            exterior_medium,
        })

    }
}
