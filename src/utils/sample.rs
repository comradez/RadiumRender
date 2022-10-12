use rand::{thread_rng, Rng};

use super::Vector3;

pub fn uniform_sphere() -> Vector3 {
    let mut rng = thread_rng();
    let theta: f64 = rng.gen_range(0.0 .. 1.0) * 2. * std::f64::consts::PI;
    let phi = f64::acos(rng.gen_range(0.0 .. 1.0) * 2. - 1.);
    Vector3::from([
        f64::cos(theta) * f64::sin(phi),
        f64::sin(theta) * f64::sin(phi),
        f64::cos(phi),
    ])
}

pub fn uniform_01() -> f64 {
    let mut rng = thread_rng();
    rng.gen_range(0.0 .. 1.0)
}

pub fn uniform_sum1() -> (f64, f64, f64) {
    let mut rng = thread_rng();
    let beta = rng.gen_range(0.0 .. 1.0);
    let gamma = rng.gen_range(0.0 .. 1.0 - beta);
    (1. - beta - gamma, beta, gamma)
}