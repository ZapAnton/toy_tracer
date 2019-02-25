use crate::gvec::Gvec;

pub struct Ray<'a> {
    pub origin: &'a Gvec,
    pub direction: &'a Gvec,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Gvec, direction: &'a Gvec) -> Self {
        Self { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f32) -> Gvec {
        self.origin + &(t * self.direction)
    }
}
