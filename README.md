# tinyraytracer-Rust

It is inspired from [tinyraytracer](https://github.com/ssloy/tinyraytracer) by [ssloy](https://github.com/ssloy) which is written in C++. This is a Rust version of it. It is a very good tutorial for beginners to learn ray tracing. I have tried to keep the code as close to the original as possible.

![Result](out.png)

## Requirements

- git
- rustc (> 1.50)
- cargo (> 1.50)

## How to compile and run

If you have `cargo` installed, you can compile and run it using:

```bash
git clone https://github.com/Qazalbash/tinyraytracer-Rust.git
cd tinyraytracer-Rust
cargo run --release
```

Otherwise, you can compile it manually using `rustc` and run the executable `tinyraytracer`:

```bash
git clone https://github.com/Qazalbash/tinyraytracer-Rust.git
cd tinyraytracer-Rust
rustc -C opt-level=3 -C target-cpu=native -C lto -C codegen-units=1 -C panic=abort -C debuginfo=0 --out-dir ./target --crate-name tinyraytracer src/main.rs
```

## Description

The code is divided into 4 files:

- [`constants.rs`](src/constants.rs): It contains the constants used in the code.
- [`main.rs`](src/main.rs): It contains the main function and the code for rendering the scene.
- [`math.rs`](src/math.rs): It contains the code for vectors.
- [`primitive.rs`](src/primitive.rs): It contains the code for primitives types like `Sphere` and `Material`.

In [`main.rs`](src/main.rs) there is a special function to print patterns on the sides. It is called `pattern`. It is just for fun. By default it is set to:

```rust
#[inline]
fn pattern(a: f32, b: f32) -> math::Vec3 {
    match ((a + 1000.0) as i32 + b as i32) & 1 == 1 {
        true => constants::DARK_SQUARE,
        false => constants::LIGHT_SQUARE,
    }
}
```

## Future work and improvements

There are many things that can be improved in this code. The first and foremost thing is to make it more optimized. I have tried to make it as optimized as possible, but there is still a lot of room for improvement. One aspect is to less use static variables and use dynamic memory allocation instead. I am also planning to make this in OpenCL and CUDA.

## Related projects

- [tinyraytracer-C](https://github.com/Qazalbash/tinyraytracer-C)

## Contributors

<a href="https://github.com/Qazalbash/tinyraytracer-Rust/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=Qazalbash/tinyraytracer-Rust" />
</a>
