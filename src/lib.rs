pub mod camera;
pub mod gvec;
pub mod ray;
pub mod sphere;

use gvec::Gvec;
use ray::Ray;
use std::borrow::Cow;

pub struct HitRecord<'a> {
    pub is_hit: bool,
    pub t: f32,
    pub p: Cow<'a, Gvec>,
    pub normal: Cow<'a, Gvec>,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

impl<'a> HitRecord<'a> {
    pub fn new(is_hit: bool, t: f32, p: Cow<'a, Gvec>, normal: Cow<'a, Gvec>) -> Self {
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
