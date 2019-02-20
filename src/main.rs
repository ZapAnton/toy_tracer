use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

fn main() -> io::Result<()> {
    let nx = 200;

    let ny = 100;

    let mut output_file = BufWriter::new(File::create("test.ppm")?);

    write!(&mut output_file, "P3\n{} {}\n255\n", nx, ny)?;

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f32 / nx as f32;

            let g = j as f32 / ny as f32;

            let b = 0.2;

            let ir = (255.99 * r) as i32;

            let ig = (255.99 * g) as i32;

            let ib = (255.99 * b) as i32;

            writeln!(&mut output_file, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}
