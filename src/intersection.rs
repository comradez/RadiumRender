use std::sync::Arc;

use crate::{
    utils::{
        Vector2,
        Vector3,
    },
    medium::Medium, material::Material,
};

pub enum Intersection {
    None, // 没有相交
    AbsorbSurface(f64), // 光线打到了单面材质的背面，保存光线起点与交点间的距离，即参数 t
    ViewPoint(Vector3), // 视点，保存视点 pos
    Medium(Vector3, Arc<dyn Medium>), // 散射点，保存位置 pos 和介质 medium
    Surface(SurfaceIntersection), // 光线和物体相交，比较复杂所以单独开 struct
}

pub struct SurfaceIntersection {
    pub position: Vector3,
    pub normal: Vector3,
    pub texcoord: Option<Vector2>,
    pub inside: bool,
    pub distance: f64,
    pub material: Arc<dyn Material>,
    pub interior_medium: Arc<dyn Medium>,
    pub exterior_medium: Arc<dyn Medium>,
}