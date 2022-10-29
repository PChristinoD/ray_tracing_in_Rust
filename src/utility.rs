use rand::Rng;

use crate::{vec3::length, Vec3};

pub const PI: f64 = 3.141592653589793238;
pub const INFI: f64 = f64::MAX;

#[inline]
pub fn degrees_to_radians(degress: f64) -> f64 {
    degress * PI / 180.0
}

#[inline]
pub fn rand() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

#[inline]
pub fn rand_slice() -> [f64; 3] {
    let mut rng = rand::thread_rng();
    [
        rng.gen::<f64>() * 2.0 - 1.0,
        rng.gen::<f64>() * 2.0 - 1.0,
        rng.gen::<f64>() * 2.0 - 1.0,
    ]
}

#[inline]
pub fn rand_unit() -> [f64; 3] {
    loop {
        let v = rand_slice();
        if v[0] * v[0] + v[1] * v[1] + v[2] * v[2] < 1.0 {
            return v;
        }
    }
}

#[inline]
pub fn rand_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(rand() * 2.0 - 1.0, rand() * 2.0 - 1.0, 0.0);
        if length(&p) < 1.0 {
            return p;
        }
    }
}
