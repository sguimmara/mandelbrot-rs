use std::path::Path;

use crate::canvas::Canvas;

pub mod complex;
pub mod canvas;

fn main() {
    println!("Hello, world!");

    let mut canvas = Canvas::new(2048, 2048);

    canvas.compute_mandelbrot_set(Path::new("mandelbrot.jpg"));
}
