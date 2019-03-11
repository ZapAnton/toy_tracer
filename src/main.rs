use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use toy_tracer::{gvec::Gvec, sphere::Sphere};

const OUTPUT_FILE_PATH: &str = "out/test.ppm";

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 768;
const FOV: f32 = std::f32::consts::PI / 2.0;

fn cast_ray(origin: &Gvec, direction: &Gvec, sphere: &Sphere) -> Gvec {
    if !sphere.intersects_ray(origin, direction, std::f32::MAX) {
        Gvec(0.2, 0.7, 0.8)
    } else {
        Gvec(0.4, 0.4, 0.3)
    }
}

fn render(sphere: &Sphere) -> io::Result<()> {
    let mut output_file = BufWriter::new(File::create(OUTPUT_FILE_PATH)?);

    writeln!(&mut output_file, "P3\n{} {}\n255", WIDTH, HEIGHT)?;

    (0..HEIGHT)
        .flat_map(|j| {
            (0..WIDTH).map(move |i| {
                let x = (2.0 * (i as f32 + 0.5) / WIDTH as f32 - 1.0)
                    * (FOV / 2.0).tan() * WIDTH as f32 / HEIGHT as f32;

                let y = -(2.0 * (j as f32 + 0.5) / HEIGHT as f32 - 1.0) * (FOV / 2.0).tan();

                let direction = Gvec(x, y, -1.0).normalize();

                cast_ray(&Gvec(0.0, 0.0, 0.0), &direction, sphere)
            })
        })
        .map(|frame| {
            (
                (255.0 * f32::max(0.0, f32::min(1.0, frame.0))) as u32,
                (255.0 * f32::max(0.0, f32::min(1.0, frame.1))) as u32,
                (255.0 * f32::max(0.0, f32::min(1.0, frame.2))) as u32,
            )
        })
        .try_for_each(|(ir, ig, ib)| writeln!(&mut output_file, "{} {} {}", ir, ig, ib))
}

fn main() -> io::Result<()> {
    let sphere = Sphere::new(Gvec(-3.0, 0.0, -16.0), 2.0);

    render(&sphere)?;

    Ok(())
}
