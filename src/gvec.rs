use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone)]
pub struct Gvec(pub f32, pub f32, pub f32);

impl Gvec {
    pub fn length(&self) -> f32 {
        (self.squared_length()).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn normalize(&self) -> Self {
        self / self.length()
    }
}

impl Add for Gvec {
    type Output = Gvec;

    fn add(self, other: Self) -> Self::Output {
        Gvec(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Add for &Gvec {
    type Output = Gvec;

    fn add(self, other: &Gvec) -> Self::Output {
        Gvec(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Gvec {
    type Output = Gvec;

    fn sub(self, other: Self) -> Self::Output {
        Gvec(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Sub for &Gvec {
    type Output = Gvec;

    fn sub(self, other: Self) -> Self::Output {
        Gvec(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Sub<&Gvec> for Gvec {
    type Output = Gvec;

    fn sub(self, other: &Gvec) -> Gvec {
        Gvec(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Div<f32> for Gvec {
    type Output = Gvec;

    fn div(self, rhs: f32) -> Self::Output {
        if rhs == 0.0 {
            panic!("Cannot divide by zero");
        }

        Gvec(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Div<f32> for &Gvec {
    type Output = Gvec;

    fn div(self, rhs: f32) -> Self::Output {
        if rhs == 0.0 {
            panic!("Cannot divide by zero");
        }

        Gvec(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Mul<f32> for Gvec {
    type Output = Gvec;

    fn mul(self, rhs: f32) -> Self::Output {
        Gvec(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Gvec> for f32 {
    type Output = Gvec;

    fn mul(self, rhs: Gvec) -> Self::Output {
        Gvec(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Mul<f32> for &Gvec {
    type Output = Gvec;

    fn mul(self, rhs: f32) -> Self::Output {
        Gvec(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<&Gvec> for f32 {
    type Output = Gvec;

    fn mul(self, rhs: &Gvec) -> Self::Output {
        Gvec(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Mul<&Gvec> for &Gvec {
    type Output = f32;

    fn mul(self, rhs: &Gvec) -> Self::Output {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
}
