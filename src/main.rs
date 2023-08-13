use std::path::Path;

use crate::canvas::Canvas;

pub mod complex;
pub mod canvas;

fn main() {
    let mut canvas = Canvas::new(4096, 4096);

    canvas.compute_mandelbrot_set(Path::new("mandelbrot.jpg"));
}
