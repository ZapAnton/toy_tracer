use std::{
    borrow::Borrow,
    fs::File,
    io::{self, BufWriter, Write},
};

use rand::{self, Rng};

use toy_tracer::{camera::Camera, gvec::Gvec, ray::Ray, sphere::Sphere, Hitable};

const OUTPUT_FILE_PATH: &str = "out/test.ppm";

const NX: i32 = 200;
const NY: i32 = 100;
const NS: i32 = 100;

fn unit_sphere_random_point() -> Gvec {
    let mut rng = rand::thread_rng();

    loop {
        let point =
            2.0 * Gvec(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - Gvec(1.0, 1.0, 1.0);

        if point.squared_length() < 1.0 {
            return point;
        }
    }
}

fn color<T: Hitable>(ray: &Ray, world: &[T]) -> Gvec {
    let (hit_record, _) = world.iter().fold(
        (Option::default(), std::f32::MAX),
        |(hit_record, closest), item| {
            if let Some(temp_record) = item.hit(ray, 0.001, closest) {
                let closest = temp_record.t;

                (Some(temp_record), closest)
            } else {
                (hit_record, closest)
            }
        },
    );

    if let Some(hit_record) = hit_record {
        let target: Gvec = hit_record.p.borrow() as &Gvec
            + hit_record.normal.borrow()
            + unit_sphere_random_point();

        let direction = target - hit_record.p.borrow();

        let ray = Ray::new(&hit_record.p, &direction);

        0.5 * color(&ray, world)
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

    let camera = Camera::default();

    let mut rng = rand::thread_rng();

    for j in (0..NY).rev() {
        for i in 0..NX {
            let col: Gvec = (0..NS)
                .map(|_| {
                    let u = (i as f32 + rng.gen::<f32>()) / NX as f32;

                    let v = (j as f32 + rng.gen::<f32>()) / NY as f32;

                    let ray_direction = camera.lower_left_corner - camera.origin
                        + u * camera.horizontal
                        + v * camera.vertical;

                    let ray = Ray::new(camera.origin, &ray_direction);

                    color(&ray, &world)
                })
                .fold(Gvec(0.0, 0.0, 0.0), |sum, color| sum + color)
                / (NS as f32);

            let ir = (255.99 * col.0.sqrt()) as u32;

            let ig = (255.99 * col.1.sqrt()) as u32;

            let ib = (255.99 * col.2.sqrt()) as u32;

            writeln!(&mut output_file, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}
