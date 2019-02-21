use crate::{dot, gvec::Gvec, ray::Ray, HitRecord, Hitable};

pub struct Sphere {
    pub radius: f32,
    pub center: Gvec,
}

impl Sphere {
    pub fn new(radius: f32, center: Gvec) -> Self {
        Self { radius, center }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;

        let a = dot(&ray.direction, &ray.direction);

        let b = 2.0 * dot(&oc, &ray.direction);

        let c = dot(&oc, &oc) - self.radius.powi(2);

        let discriminant = b.powi(2) - a * c;

        if discriminant.is_sign_positive() {
            let temp = (-b - discriminant.sqrt()) / a;

            if (temp < t_max) && (temp > t_min) {
                let p = ray.point_at_parameter(temp);

                return Some(HitRecord::new(
                    true,
                    temp,
                    p,
                    (p - self.center) / self.radius,
                ));
            }

            let temp = (-b + discriminant.sqrt()) / a;

            if (temp < t_max) && (temp > t_min) {
                let p = ray.point_at_parameter(temp);

                return Some(HitRecord::new(
                    true,
                    temp,
                    p,
                    (p - self.center) / self.radius,
                ));
            }
        }

        None
    }
}
