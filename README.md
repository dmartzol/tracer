# Tracer

`Tracer` is a CPU based ray tracer writen in Rust.

I wrote this with the intention of learning Rust and create generative art with it.

## Usage

Right now it only supports ppm images. It outputs the content of the ppm image to stdout, so to generate an image you can do:

```
git clone https://github.com/dmartzol/tracer
cd tracer
cargo run --release src/main.rs > spheres.ppm
```

## Notes

Right now it only supports spheres.

It supports 3 different materials:
  - Lambertian (diffuse material)
  - Dielectric (glass material)
  - Metalic

