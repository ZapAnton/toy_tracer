use crate::gvec::Gvec;

#[derive(Default, Clone)]
pub struct Material {
    pub diffuse_color: Gvec,
}

impl Material {
    pub fn new(diffuse_color: Gvec) -> Self {
        Self { diffuse_color }
    }
}

pub struct Sphere {
    pub radius: f32,
    pub center: Gvec,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Gvec, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn intersects_ray(&self, origin: &Gvec, direction: &Gvec) -> Option<f32> {
        let L = &self.center - origin;

        let tca = &L * direction;

        let d2 = &L * &L - tca.powi(2);

        if d2 > self.radius.powi(2) {
            return None;
        }

        let thc = (self.radius.powi(2) - d2).sqrt();

        let mut t0 = tca - thc;

        let t1 = tca + thc;

        if t0 < 0.0 {
            t0 = t1;
        }

        if t0 < 0.0 {
            None
        } else {
            Some(t0)
        }
    }
}
