use crate::gvec::Gvec;
use std::default::Default;

pub struct Camera<'a> {
    pub origin: &'a Gvec,
    pub lower_left_corner: &'a Gvec,
    pub horizontal: &'a Gvec,
    pub vertical: &'a Gvec,
}

impl<'a> Default for Camera<'a> {
    fn default() -> Self {
        Self {
            origin: &Gvec(0.0, 0.0, 0.0),
            lower_left_corner: &Gvec(-2.0, -1.0, -1.0),
            horizontal: &Gvec(4.0, 0.0, 0.0),
            vertical: &Gvec(0.0, 2.0, 0.0),
        }
    }
}
