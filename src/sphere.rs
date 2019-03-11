use crate::gvec::Gvec;

pub struct Sphere {
    pub radius: f32,
    pub center: Gvec,
}

impl Sphere {
    pub fn new(center: Gvec, radius: f32) -> Self {
        Sphere { center, radius }
    }

    pub fn intersects_ray(&self, origin: &Gvec, direction: &Gvec, t0: f32) -> bool {
        let L = &self.center - origin;

        let tca = &L * direction;

        let d2 = &L * &L - tca.powi(2);

        if d2 > self.radius.powi(2) {
            return false;
        }

        let thc = (self.radius.powi(2) - d2).sqrt();

        let mut t0 = tca - thc;

        let t1 = tca + thc;

        if t0 < 0.0 {
            t0 = t1;
        }

        if t0 < 0.0 {
            false
        } else {
            true
        }
    }
}
