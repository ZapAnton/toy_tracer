mod gvec;

use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use gvec::Gvec;

const NX: i32 = 200;
const NY: i32 = 100;
const OUTPUT_FILE_PATH: &str = "out/test.ppm";

fn main() -> io::Result<()> {
    let mut output_file = BufWriter::new(File::create(OUTPUT_FILE_PATH)?);

    write!(&mut output_file, "P3\n{} {}\n255\n", NX, NY)?;

    for j in (0..NY).rev() {
        for i in 0..NX {
            let col = Gvec::new(i as f32 / NX as f32, j as f32 / NY as f32, 0.2);

            let ir = (255.99 * col.e[0]) as i32;

            let ig = (255.99 * col.e[1]) as i32;

            let ib = (255.99 * col.e[2]) as i32;

            writeln!(&mut output_file, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}
