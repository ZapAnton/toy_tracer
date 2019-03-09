use crate::{gvec::Gvec, ray::Ray, HitRecord};
use std::borrow::Borrow;

pub struct ScatteredRay<'a, 'b> {
    pub ray: Ray<'a>,
    pub attenuation: &'b Gvec,
}

pub enum Material {
    Lambertian(Gvec),
    Metal(Gvec),
}

fn reflect(v: &Gvec, n: &Gvec) -> Gvec {
    v - &(2.0 * crate::dot(v, n) * n)
}

impl Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        match self {
            Material::Lambertian(albedo) => {
                let target: Gvec = hit_record.p.borrow() as &Gvec
                    + hit_record.normal.borrow()
                    + crate::unit_sphere_random_point();
                Some(ScatteredRay {
                    ray: Ray::new(&hit_record.p, &(target - hit_record.p.borrow())),
                    attenuation: albedo,
                })
            }
            Material::Metal(albedo) => {
                let reflected = reflect(&ray_in.direction.unit(), &hit_record.normal);
                let ray = Ray::new(&hit_record.p, &reflected);
                if crate::dot(ray.direction, &hit_record.normal) > 0.0 {
                    Some(ScatteredRay {
                        ray: ray,
                        attenuation: albedo,
                    })
                } else {
                    None
                }
            }
        }
    }
}
