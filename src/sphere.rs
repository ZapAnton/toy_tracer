use crate::{dot, gvec::Gvec, ray::Ray, HitRecord, Hitable};
use std::borrow::Cow;

pub struct Sphere<'a> {
    pub radius: f32,
    pub center: &'a Gvec,
}

impl<'a> Sphere<'a> {
    pub fn new(radius: f32, center: &'a Gvec) -> Self {
        Self { radius, center }
    }
}

impl<'a> Hitable for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;

        let a = dot(&ray.direction, &ray.direction);

        let b = dot(&oc, &ray.direction);

        let c = dot(&oc, &oc) - self.radius.powi(2);

        let discriminant = b.powi(2) - a * c;

        if discriminant.is_sign_positive() {
            let temp = (-b - discriminant.sqrt()) / a;

            if (temp < t_max) && (temp > t_min) {
                let p = ray.point_at_parameter(temp);

                let normal = (&p - self.center) / self.radius;

                return Some(HitRecord::new(
                    true,
                    temp,
                    Cow::Owned(p),
                    Cow::Owned(normal),
                ));
            }

            let temp = (-b + discriminant.sqrt()) / a;

            if (temp < t_max) && (temp > t_min) {
                let p = ray.point_at_parameter(temp);

                let normal = (&p - self.center) / self.radius;

                return Some(HitRecord::new(
                    true,
                    temp,
                    Cow::Owned(p),
                    Cow::Owned(normal),
                ));
            }
        }

        None
    }
}
