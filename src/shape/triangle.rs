use std::{f64::{consts::PI, EPSILON}, sync::Arc};
use vecmat::traits::Dot;

use crate::{utils::{Vector3, Matrix3, Matrix4, Vector2, uniform_01, uniform_sum1}, ray::Ray, intersection::{Intersection, SurfaceIntersection}, medium::Medium, material::Material};

use super::IntersectWithRay;

pub struct Triangle {
    vertices: [Vector3; 3],
    texcoords: Option<[Vector2; 3]>,
    normals: [Vector3; 3], // 法向, 如果有点法向则保存点法向, 否则用传入的 vertice 的顺序计算面法向
    tangents: Option<[Vector3; 3]>, // 切向，用于 Normal Perturbing
    bitangents: Option<[Vector3; 3]>, // 用于 Normal Perturbing
    v0v1: Vector3,
    v0v2: Vector3,
}

impl Triangle {
    pub fn new(
        vertices: [Vector3; 3],
        texcoords: Option<[Vector2; 3]>,
        normals: Option<[Vector3; 3]>, // 点法向
        tangents: Option<[Vector3; 3]>, // 切向，用于 Normal Perturbing
        bitangents: Option<[Vector3; 3]>, // 用于 Normal Perturbing
    ) -> Self {
        let v0v1 = vertices[1] - vertices[0];
        let v0v2 = vertices[2] - vertices[0];
        let surface_normal = v0v1.cross(v0v2).normalize();
        let normals = normals.unwrap_or([surface_normal; 3]);
        Self {
            vertices,
            texcoords,
            normals,
            tangents,
            bitangents,
            v0v1,
            v0v2,
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
        let mat = Matrix3::from_array_of_vectors([
            self.v0v1,
            self.v0v2,
            -ray.direction,
        ]).transpose(); // det 为负表明打在了背面
        let det = mat.det();
        if det.abs() > EPSILON {
            let x = mat.inv().dot(ray.origin - self.vertices[0]);
            let (u, v, t) = (x[0], x[1], x[2]);

            if t > t_min && 0. < u && 0. < v && u + v < 1. {
                if !material.two_sided() && (flip_normals && det > 0. || !flip_normals && det < 0.) {
                    return Intersection::AbsorbSurface(t)
                } // 交在了单面材质的背面，直接吸收
                let point = ray.point_at_param(t);
                let distance = (point - ray.origin).length();
                let (alpha, beta, gamma) = (1. - u - v, u, v);
                let position = self.vertices[0] * alpha + self.vertices[1] * beta + self.vertices[2] * gamma;
                let mut normal = (self.normals[0] * alpha + self.normals[1] * beta + self.normals[2] * gamma).normalize();
                let mut inside = false;

                if det < 0. {
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
                Intersection::None // 交在了平面内、三角形外，或者 t 值不对
            }
        } else {
            Intersection::None // 平行
        }
    }

    pub fn sample(
        &self,
        to_world: &Option<Matrix4>,
        material: Arc<dyn Material>,
        interior_medium: Arc<dyn Medium>,
        exterior_medium: Arc<dyn Medium>,
    ) -> Intersection {
        let (alpha, beta, gamma) = uniform_sum1();
        let position = self.vertices[0] * alpha + self.vertices[1] * beta + self.vertices[2] * gamma;
        let normal = (self.normals[0] * alpha + self.normals[1] * beta + self.normals[2] * gamma).normalize();
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