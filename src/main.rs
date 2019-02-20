mod gvec;

use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use gvec::Gvec;

fn main() -> io::Result<()> {
    let nx = 200;

    let ny = 100;

    let mut output_file = BufWriter::new(File::create("test.ppm")?);

    write!(&mut output_file, "P3\n{} {}\n255\n", nx, ny)?;

    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Gvec::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2);

            let ir = (255.99 * col.e[0]) as i32;

            let ig = (255.99 * col.e[1]) as i32;

            let ib = (255.99 * col.e[2]) as i32;

            writeln!(&mut output_file, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}
