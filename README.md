# GPX Tools

My own tools for manipulating GPX files. Written so it's easier to create proper GPX files for running based on a series of not-so-perfect GPX sources.

## Usage

Elevation command:

```
[cargo run --] elevation --route ~/Dropbox/_temp/maps/nychalf-2023/map-course.gpx --elevation '~/Dropbox/_temp/maps/nychalf-2023/*.gpx'
```

## Dev

Build

```
cargo build
```

Run

```
cargo run
```

Run tests

```
cargo test
```

## Building References

* [Command Line Applications in Rust - In-depth topics](https://rust-cli.github.io/book/in-depth/index.html)
* [StructOpt](https://docs.rs/structopt/latest/structopt/)
* [GPX](https://crates.io/crates/gpx)
