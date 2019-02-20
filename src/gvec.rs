/*pub struct Gvec {
    pub e: [f32; 3],
}

impl Gvec {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Self { e: [e0, e1, e2] }
    }
}*/

pub struct Gvec(pub f32, pub f32, pub f32);

impl Gvec {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Self(e0, e1, e2)
    }
}
