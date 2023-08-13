use std::path::Path;

use image::{Rgb, ImageBuffer};

use crate::complex::Complex;

const DOMAIN: (f64, f64) = (-3.0, 3.0);

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<bool>,
}

const ITERATIONS: usize = 20;

fn is_in_mandelbrot_set(c: Complex) -> bool {
    let mut accum = c;

    for _ in 0..ITERATIONS {
        accum = (accum * accum) + c;
    }

    return accum.magnitude() < 2.0;
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            pixels: vec![false; size],
        }
    }

    fn write_to(&mut self, path: &Path) {
        // a default (black) image containing Rgb values
        let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(self.width as u32, self.height as u32);

        for y in 0..self.height {
            for x in 0..self.width {
                let active = self.pixels[y * self.width + x];
                if active {
                    *image.get_pixel_mut(x as u32, y as u32) = image::Rgb([255, 255, 255]);
                }
            }
        }

        image.save(path).unwrap();
    }

    pub fn compute_mandelbrot_set(&mut self, path: &Path) {
        self.do_compute_loops();

        self.write_to(path)
    }

    fn compute_pixel(&mut self, x: usize, y: usize) {
        let x_step = 1.0 / self.width as f64;
        let y_step = 1.0 / self.height as f64;
        let dom = DOMAIN.1 - DOMAIN.0;

        let r = DOMAIN.0 + (x as f64 * x_step) * dom;
        let i = DOMAIN.0 + (y as f64 * y_step) * dom;
        let c = Complex::new(r, i);
        if is_in_mandelbrot_set(c) {
            self.pixels[y * self.width + x] = true;
        }
    }

    fn do_compute_iter(&mut self) {
        let size = self.width * self.height;

        (0..size).for_each(|pos| {
            let x = pos % self.width;
            let y = pos / self.height;

            self.compute_pixel(x, y);
        });
    }

    fn do_compute_loops(&mut self) {
        let x_step = 1.0 / self.width as f64;
        let y_step = 1.0 / self.height as f64;
        let dom = DOMAIN.1 - DOMAIN.0;

        for y in 0..self.height {
            for x in 0..self.width {
                let r = DOMAIN.0 + (x as f64 * x_step) * dom;
                let i = DOMAIN.0 + (y as f64 * y_step) * dom;
                let c = Complex::new(r, i);
                if is_in_mandelbrot_set(c) {
                    self.pixels[y * self.width + x] = true;
                }
            }
        }
    }
}
