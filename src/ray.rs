use crate::gvec::Gvec;

pub struct Ray {
    pub origin: Gvec,
    pub direction: Gvec,
}

impl Ray {
    pub fn new(origin: Gvec, direction: Gvec) -> Self {
        Self { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f32) -> Gvec {
        self.origin.clone() + t * self.direction.clone()
    }
}
