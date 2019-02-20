mod gvec;
mod ray;

use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use gvec::Gvec;
use ray::Ray;

const OUTPUT_FILE_PATH: &str = "out/test.ppm";

const NX: i32 = 200;
const NY: i32 = 100;
const LOWER_LEFT_CORNER: Gvec = Gvec(-2.0, -1.0, -1.0);
const HORIZONTAL: Gvec = Gvec(4.0, 0.0, 0.0);
const VERTICAL: Gvec = Gvec(0.0, 2.0, 0.0);
const ORIGIN: Gvec = Gvec(0.0, 0.0, 0.0);

fn dot(v1: &Gvec, v2: &Gvec) -> f32 {
    v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
}

fn hits_sphere(center: &Gvec, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin.clone() - center.clone();

    let a = dot(&ray.direction, &ray.direction);

    let b = 2.0 * dot(&oc, &ray.direction);

    let c = dot(&oc, &oc) - radius.powi(2);

    let discriminant = b.powi(2) - 4.0 * a * c;

    discriminant.is_sign_positive()
}

fn color(ray: &Ray) -> Gvec {
    if hits_sphere(&Gvec(0.0, 0.0, -1.0), 0.5, ray) {
        return Gvec(1.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction.unit();

    let t = 0.5 * (unit_direction.1 + 1.0);

    (1.0 - t) * Gvec(1.0, 1.0, 1.0) + t * Gvec(0.5, 0.7, 1.0)
}

fn main() -> io::Result<()> {
    let mut output_file = BufWriter::new(File::create(OUTPUT_FILE_PATH)?);

    write!(&mut output_file, "P3\n{} {}\n255\n", NX, NY)?;

    for j in (0..NY).rev() {
        for i in 0..NX {
            let u = i as f32 / NX as f32;

            let v = j as f32 / NY as f32;

            let ray = Ray::new(ORIGIN, LOWER_LEFT_CORNER + u * HORIZONTAL + v * VERTICAL);

            let col = color(&ray);

            let ir = (255.99 * col.0) as i32;

            let ig = (255.99 * col.1) as i32;

            let ib = (255.99 * col.2) as i32;

            writeln!(&mut output_file, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}
