use std::path::Path;

use crate::canvas::Canvas;

pub mod complex;
pub mod canvas;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The width of the generated image, in pixels.
    #[arg(short, long, default_value_t = 512)]
    width: usize,

    /// The height of the generated image, in pixels.
    #[arg(short, long, default_value_t = 512)]
    height: usize,

    /// Number of iterations of each computation
    #[arg(short, long, default_value_t = 10)]
    iterations: u8,

    /// Output file
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut canvas = Canvas::new(args.width, args.height);

    canvas.compute_mandelbrot_set(Path::new(&args.output.unwrap_or("mandelbrot.jpg".to_string())));
}
