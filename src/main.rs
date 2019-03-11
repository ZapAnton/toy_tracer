use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use toy_tracer::gvec::Gvec;

const OUTPUT_FILE_PATH: &str = "out/test.ppm";

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 768;

fn main() -> io::Result<()> {
    let mut output_file = BufWriter::new(File::create(OUTPUT_FILE_PATH)?);

    writeln!(&mut output_file, "P3\n{} {}\n255", WIDTH, HEIGHT)?;

    (0..HEIGHT)
        .flat_map(|height| {
            (0..WIDTH).map(move |width| {
                Gvec(
                    height as f32 / HEIGHT as f32,
                    width as f32 / WIDTH as f32,
                    0.0,
                )
            })
        })
        .map(|frame| {
            (
                (255.0 * f32::max(0.0, f32::min(1.0, frame.0))) as u32,
                (255.0 * f32::max(0.0, f32::min(1.0, frame.1))) as u32,
                (255.0 * f32::max(0.0, f32::min(1.0, frame.2))) as u32,
            )
        })
        .try_for_each(|(ir, ig, ib)| writeln!(&mut output_file, "{} {} {}", ir, ig, ib))?;

    Ok(())
}
