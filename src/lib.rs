pub mod gvec;
pub mod ray;
pub mod sphere;

use gvec::Gvec;
use ray::Ray;

pub struct HitRecord {
    pub is_hit: bool,
    pub t: f32,
    pub p: Gvec,
    pub normal: Gvec,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new(is_hit: bool, t: f32, p: Gvec, normal: Gvec) -> Self {
        Self {
            is_hit,
            t,
            p,
            normal,
        }
    }
}

pub fn dot(v1: &Gvec, v2: &Gvec) -> f32 {
    v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
}
