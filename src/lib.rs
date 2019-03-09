pub mod camera;
pub mod gvec;
pub mod material;
pub mod ray;
pub mod sphere;

use gvec::Gvec;
use material::Material;
use rand::{self, Rng};
use ray::Ray;
use std::borrow::Cow;

pub struct HitRecord<'a> {
    pub is_hit: bool,
    pub t: f32,
    pub p: Cow<'a, Gvec>,
    pub normal: Cow<'a, Gvec>,
    pub material: Material,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

impl<'a> HitRecord<'a> {
    pub fn new(
        is_hit: bool,
        t: f32,
        p: Cow<'a, Gvec>,
        normal: Cow<'a, Gvec>,
        material: Material,
    ) -> Self {
        Self {
            is_hit,
            t,
            p,
            normal,
            material,
        }
    }
}

pub fn dot(v1: &Gvec, v2: &Gvec) -> f32 {
    v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
}

pub fn unit_sphere_random_point() -> Gvec {
    let mut rng = rand::thread_rng();

    loop {
        let point =
            2.0 * Gvec(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - Gvec(1.0, 1.0, 1.0);

        if point.squared_length() < 1.0 {
            return point;
        }
    }
}
