use std::ops::{Add, Mul};
pub struct Gvec(pub f32, pub f32, pub f32);

impl Gvec {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Self(e0, e1, e2)
    }
}

impl Add for Gvec {
    type Output = Gvec;

    fn add(self, other: Self) -> Self::Output {
        Gvec(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Mul<f32> for Gvec {
    type Output = Gvec;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Gvec> for f32 {
    type Output = Gvec;

    fn mul(self, rhs: Gvec) -> Self::Output {
        Gvec(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}
