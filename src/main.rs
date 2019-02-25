use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use toy_tracer::{gvec::Gvec, ray::Ray, sphere::Sphere, HitRecord, Hitable};

const OUTPUT_FILE_PATH: &str = "out/test.ppm";

const NX: i32 = 200;
const NY: i32 = 100;
const LOWER_LEFT_CORNER: Gvec = Gvec(-2.0, -1.0, -1.0);
const HORIZONTAL: Gvec = Gvec(4.0, 0.0, 0.0);
const VERTICAL: Gvec = Gvec(0.0, 2.0, 0.0);
const ORIGIN: Gvec = Gvec(0.0, 0.0, 0.0);

fn color<T: Hitable>(ray: &Ray, world: &[T]) -> Gvec {
    let mut hit_record: Option<HitRecord> = Option::default();

    let mut closest = std::f32::MAX;

    for item in world.iter() {
        if let Some(temp_record) = item.hit(ray, 0.0, closest) {
            closest = temp_record.t;
            hit_record = Some(temp_record);
        }
    }

    if let Some(hit_record) = hit_record {
        0.5 * Gvec(
            hit_record.normal.0 + 1.0,
            hit_record.normal.1 + 1.0,
            hit_record.normal.2 + 1.0,
        )
    } else {
        let unit_direction = ray.direction.unit();

        let t = 0.5 * (unit_direction.1 + 1.0);

        (1.0 - t) * Gvec(1.0, 1.0, 1.0) + t * Gvec(0.5, 0.7, 1.0)
    }
}

fn main() -> io::Result<()> {
    let mut output_file = BufWriter::new(File::create(OUTPUT_FILE_PATH)?);

    write!(&mut output_file, "P3\n{} {}\n255\n", NX, NY)?;

    let world = [
        Sphere::new(0.5, &Gvec(0.0, 0.0, -1.0)),
        Sphere::new(100.0, &Gvec(0.0, -100.5, -1.0)),
    ];

    for j in (0..NY).rev() {
        for i in 0..NX {
            let u = i as f32 / NX as f32;

            let v = j as f32 / NY as f32;

            let ray_direction = LOWER_LEFT_CORNER + u * HORIZONTAL + v * VERTICAL;

            let ray = Ray::new(&ORIGIN, &ray_direction);

            let col = color(&ray, &world);

            let ir = (255.99 * col.0) as u32;

            let ig = (255.99 * col.1) as u32;

            let ib = (255.99 * col.2) as u32;

            writeln!(&mut output_file, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}
