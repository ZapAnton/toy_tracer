use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use toy_tracer::{
    gvec::Gvec,
    sphere::{Light, Material, Sphere},
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
    let (distance, hit, n, material) = spheres.iter().fold(
        (
            std::f32::MAX,
            Gvec::default(),
            Gvec::default(),
            Material::default(),
        ),
        |(distance, hit, n, material), sphere| {
            if let Some(sphere_dist) = sphere.intersects_ray(origin, direction) {
                if sphere_dist < distance {
                    (
                        sphere_dist,
                        origin + &(direction * sphere_dist),
                        (&hit - &sphere.center).normalize(),
                        sphere.material.clone(),
                    )
                } else {
                    (distance, hit, n, material)
                }
            } else {
                (distance, hit, n, material)
            }
        },
    );

    if distance < 1000.0 {
        Some((hit, n, material))
    } else {
        None
    }
}

fn reflect(i: &Gvec, n: &Gvec) -> Gvec {
    i - &(n * 2.0 * (i * n))
}

fn cast_ray(origin: &Gvec, direction: &Gvec, spheres: &[Sphere], lights: &[Light]) -> Gvec {
    if let Some((point, n, material)) = scene_intersect(origin, direction, spheres) {
        let (diffuse_light_intensity, specular_light_intensity) = lights
            .iter()
            .fold(
                (0.0, 0.0),
                |(diffuse_light_intensity, specular_light_intensity), light| {
                    let light_direction = (&light.position - &point).normalize();

                    (
                        diffuse_light_intensity
                            + light.intensity * f32::max(0.0, &light_direction * &n),

                        specular_light_intensity
                            + f32::max(
                                0.0,
                                &(-1.0 * reflect(&(-1.0 * &light_direction), &n)) * direction,
                            )
                            .powf(material.specular_exponent)
                                * light.intensity,
                    )
                },
            );

        &material.diffuse_color * diffuse_light_intensity * material.albedo[0]
            + &Gvec(1.0, 1.0, 1.0) * specular_light_intensity * material.albedo[1]
    } else {
        Gvec(0.2, 0.7, 0.8)
    }
}

fn render(spheres: &[Sphere], lights: &[Light]) -> io::Result<()> {
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

                cast_ray(&Gvec(0.0, 0.0, 0.0), &direction, spheres, lights)
            })
        })
        .map(|frame| {
            let max = f32::max(frame.0, f32::max(frame.1, frame.2));
            let frame = if max > 1.0 {
                frame * (1.0 / max)
            } else {
                frame
            };

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
            Material::new([0.6, 0.3], Gvec(0.4, 0.4, 0.3), 50.0),
        ),
        Sphere::new(
            Gvec(-1.0, -1.5, -12.0),
            2.0,
            Material::new([0.9, 0.1], Gvec(0.3, 0.1, 0.1), 10.0),
        ),
        Sphere::new(
            Gvec(1.5, -0.5, -18.0),
            3.0,
            Material::new([0.9, 0.1], Gvec(0.3, 0.1, 0.1), 10.0),
        ),
        Sphere::new(
            Gvec(7.0, 5.0, -18.0),
            4.0,
            Material::new([0.6, 0.3], Gvec(0.4, 0.4, 0.3), 50.0),
        ),
    ];

    let lights = [
        Light::new(Gvec(-20.0, 20.0, 20.0), 1.5),
        Light::new(Gvec(30.0, 50.0, -25.0), 1.8),
        Light::new(Gvec(30.0, 20.0, 30.0), 1.7),
    ];

    render(&spheres, &lights)?;

    Ok(())
}
