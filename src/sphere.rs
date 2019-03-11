use crate::gvec::Gvec;

#[derive(Clone)]
pub struct Material {
    pub diffuse_color: Gvec,
    pub albedo: [f32; 2],
    pub specular_exponent: f32,
}

pub struct Light {
    pub position: Gvec,
    pub intensity: f32,
}

impl Material {
    pub fn new(albedo: [f32; 2], diffuse_color: Gvec, specular_exponent: f32) -> Self {
        Self {
            albedo,
            diffuse_color,
            specular_exponent,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            albedo: [1.0, 0.0],
            diffuse_color: Gvec::default(),
            specular_exponent: f32::default(),
        }
    }
}

impl Light {
    pub fn new(position: Gvec, intensity: f32) -> Self {
        Self {
            position,
            intensity,
        }
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
        let l = &self.center - origin;

        let tca = &l * direction;

        let d2 = &l * &l - tca.powi(2);

        if d2 > self.radius.powi(2) {
            return None;
        }

        let thc = (self.radius.powi(2) - d2).sqrt();

        let mut t0 = tca - thc;

        if t0 < 0.0 {
            t0 = tca + thc;
        }

        if t0 < 0.0 {
            None
        } else {
            Some(t0)
        }
    }
}
