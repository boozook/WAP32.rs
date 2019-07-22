# WAP

WAP is a library for support original ressources for WAP32 engine.


[Prepare ressources](tests/data/README.md).



## History of Project

[First version][] was written on Haxe lang as draft implementation following to results of mindblowing reverse-engineering of th original game.

[Second version][] is reimplementation on Rust.

Checkout the [entire project][] [there][entire project].

[First version]: https://bitbucket.org/Re-Gruntz/hx.gruntz/src/develop/
[Second version]: https://bitbucket.org/Re-Gruntz/wap/src/develop/
[entire project]: https://bitbucket.org/account/user/Re-Gruntz/projects/GRUNTZ


- - -


## How to build

1. Install Rust via [RustUp](http://rustup.rs) ([intro-doc](https://github.com/rust-lang-nursery/rustup.rs/blob/master/README.md)).
1. `rustup install nightly`
1. __`cargo +nightly build`__ or `rustup run nightly cargo build` if prev step was omitted.

_NOTE:_ currently nightly isn't needed, bust build with actual stable rustc.


## How to run tests

`cargo test` or `cargo test specified_test`.

[Run tests on iOS](tests/data/README.md)


## How to run examples

- `cargo build --example tiles-rgb`
- `cargo build --example unique-tiles`

