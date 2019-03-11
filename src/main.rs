use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use toy_tracer::{
    gvec::Gvec,
    sphere::{Material, Sphere},
};

const OUTPUT_FILE_PATH: &str = "out/test.ppm";

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 768;
const FOV: f32 = std::f32::consts::PI / 2.0;

fn scene_intersect(
    origin: &Gvec,
    direction: &Gvec,
    spheres: &[Sphere],
) -> Option<(Gvec, Gvec, Material)> {
    let (distance, hit, N, material) = spheres.iter().fold(
        (
            std::f32::MAX,
            Gvec::default(),
            Gvec::default(),
            Material::default(),
        ),
        |(distance, hit, N, material), sphere| {
            if let Some(sphere_dist) = sphere.intersects_ray(origin, direction) {
                if sphere_dist < distance {
                    (
                        sphere_dist,
                        origin + &(direction * sphere_dist),
                        (hit - sphere.center.clone()).normalize(),
                        sphere.material.clone(),
                    )
                } else {
                    (distance, hit, N, material)
                }
            } else {
                (distance, hit, N, material)
            }
        },
    );

    if distance < 1000.0 {
        Some((hit, N, material))
    } else {
        None
    }
}

fn cast_ray(origin: &Gvec, direction: &Gvec, spheres: &[Sphere]) -> Gvec {
    if let Some((_point, _N, material)) = scene_intersect(origin, direction, spheres) {
        material.diffuse_color
    } else {
        Gvec(0.2, 0.7, 0.8)
    }
}

fn render(spheres: &[Sphere]) -> io::Result<()> {
    let mut output_file = BufWriter::new(File::create(OUTPUT_FILE_PATH)?);

    writeln!(&mut output_file, "P3\n{} {}\n255", WIDTH, HEIGHT)?;

    (0..HEIGHT)
        .flat_map(|j| {
            (0..WIDTH).map(move |i| {
                let x = (2.0 * (i as f32 + 0.5) / WIDTH as f32 - 1.0)
                    * (FOV / 2.0).tan()
                    * WIDTH as f32
                    / HEIGHT as f32;

                let y = -(2.0 * (j as f32 + 0.5) / HEIGHT as f32 - 1.0) * (FOV / 2.0).tan();

                let direction = Gvec(x, y, -1.0).normalize();

                cast_ray(&Gvec(0.0, 0.0, 0.0), &direction, spheres)
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
    let spheres = [
        Sphere::new(
            Gvec(-3.0, 0.0, -16.0),
            2.0,
            Material::new(Gvec(0.4, 0.4, 0.3)),
        ),
        Sphere::new(
            Gvec(-1.0, -1.5, -12.0),
            2.0,
            Material::new(Gvec(0.3, 0.1, 0.1)),
        ),
        Sphere::new(
            Gvec(1.5, -0.5, -18.0),
            3.0,
            Material::new(Gvec(0.3, 0.1, 0.1)),
        ),
        Sphere::new(
            Gvec(7.0, 5.0, -18.0),
            4.0,
            Material::new(Gvec(0.4, 0.4, 0.3)),
        ),
    ];

    render(&spheres)?;

    Ok(())
}
