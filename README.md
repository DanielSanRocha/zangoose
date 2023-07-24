# Zangoose

<img src="zangoose.jpeg" width="300" height="300"/>

Zangoose is a jack audio application written in Rust for denoising the signal and applying some effects.

## How to Install

If on linux, you can go to the release [page](https://github.com/DanielSanRocha/zangoose/releases) and download the binary of the latest version. In other OS install it from source.

## Installing from source

Clone the repository and run
```bash
cargo build --release
```
the binary of the application will be available at the target/release folder.

## Effects

Currently the only effect implemented is a overdrive effect using hiperbolic tangent.

## Denoise Feature

Zangoose has a denoise algorithm, very useful for single coil guitars.

## Acknowledgments

Made with love by Daniel Santana
