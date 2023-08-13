# mandelbrot-rs

A simple Mandelbrot set image generator written in Rust

## How to use

Run this command to get the help.

```shell
cargo run --release -- --help
```

To generate a 1024x1024 pixels image into `myImage.jpg`:

```shell
cargo run --release -- -w 1024 -h 1024 -o myImage.jpg
```
