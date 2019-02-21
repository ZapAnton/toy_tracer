pub mod gvec;
pub mod ray;

use gvec::Gvec;
use ray::Ray;

fn dot(v1: &Gvec, v2: &Gvec) -> f32 {
    v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
}

fn sphere_hit_point(center: &Gvec, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin.clone() - center.clone();

    let a = dot(&ray.direction, &ray.direction);

    let b = 2.0 * dot(&oc, &ray.direction);

    let c = dot(&oc, &oc) - radius.powi(2);

    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant.is_sign_negative() {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

pub fn color(ray: &Ray) -> Gvec {
    let hit_point = sphere_hit_point(&Gvec(0.0, 0.0, -1.0), 0.5, ray);

    if hit_point.is_sign_positive() {
        let n = (ray.point_at_parameter(hit_point) - Gvec(0.0, 0.0, -1.0)).unit();

        return 0.5 * Gvec(n.0 + 1.0, n.1 + 1.0, n.2 + 1.0);
    }

    let unit_direction = ray.direction.unit();

    let t = 0.5 * (unit_direction.1 + 1.0);

    (1.0 - t) * Gvec(1.0, 1.0, 1.0) + t * Gvec(0.5, 0.7, 1.0)
}
