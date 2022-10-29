use vectrix::{row_vector, vector, RowVector, Vector};

pub type Vec3 = Vector<f64, 3>;
pub type RowVec3 = RowVector<f64, 3>;

pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
    vector!(x, y, z)
}

pub fn new_with_number(t: f64) -> Vec3 {
    new(t, t, t)
}

pub fn new_with_x(x: f64) -> Vec3 {
    new(x, 0_f64, 0_f64)
}
pub fn new_with_y(y: f64) -> Vec3 {
    new(0_f64, y, 0_f64)
}
pub fn new_with_z(z: f64) -> Vec3 {
    new(0_f64, 0_f64, z)
}

pub fn length(vec3: &Vec3) -> f64 {
    vec3.iter().fold(0.0, |sum, x| sum + x * x).sqrt()
}

pub fn unit(vec3: &Vec3) -> Vec3 {
    let factor = length(vec3);
    new(vec3.x / factor, vec3.y / factor, vec3.z / factor)
}
pub fn mul(&lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3::new(lhs.x * rhs.x, lhs.y * rhs.y, lhs.z * rhs.z)
}

pub fn cross(&lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    let a1 = lhs.x;
    let a2 = lhs.y;
    let a3 = lhs.z;
    let b1 = rhs.x;
    let b2 = rhs.y;
    let b3 = rhs.z;
    Vec3::new(a2 * b3 - a3 * b2, a3 * b1 - a1 * b3, a1 * b2 - a2 * b1)
}
pub fn near_zero(vec3: &Vec3) -> bool {
    let ep = 1e-8;
    if vec3.x.abs() < ep && vec3.y.abs() < ep && vec3.z.abs() < ep {
        return true;
    }
    false
}

pub fn inverse(vec3: &Vec3) -> RowVec3 {
    row_vector!(vec3.x, vec3.y, vec3.z)
}
