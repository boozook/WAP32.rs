# WAP

WAP is a library for support original ressources for WAP32 engine.


[Prepare ressources](tests/data/README.md).






- - -

## How to build

1. Install Rust via [RustUp](http://rustup.rs) ([intro-doc](https://github.com/rust-lang-nursery/rustup.rs/blob/master/README.md)).
1. `rustup install nightly`
1. `rustup default nightly` (optional)
1. __`cargo build`__ or `rustup run nightly cargo build` if prev step was omitted.


## How to run tests

`cargo test` or `cargo test specified_test`.

[Run tests on iOS](tests/data/README.md)


## How to run examples

- `cargo build --example tiles-rgb`
- `cargo build --example unique-tiles`

