# clingo-rs
Rust bindings to the [clingo](https://github.com/potassco/clingo) library.
Clingo version 5.2.2.

## Requirements

- a c++14 conforming compiler
  - *at least* [gcc](https://gcc.gnu.org/) version 4.9
  - [clang](http://clang.llvm.org/) version 3.1 (using either libstdc++
    provided by gcc 4.9 or libc++)

## Compile & Test
    cargo build
    cargo test
    cargo run --example=ast 0
    cargo run --example=backend 0
    cargo run --example=configuration
    cargo run --example=control 0
    cargo run --example=model 0
    cargo run --example=propagator 0
    cargo run --example=solve-async 0
    cargo run --example=statistics 0
    cargo run --example=symbol 0
    cargo run --example=symbolic-atoms 0
    cargo run --example=theory-atoms 0
    cargo run --example=inject-terms 0
    cargo run --example=version

## Contribution

Any contribution intentionally submitted for inclusion in the work by you, shall be licensed under the terms of the MIT license without any additional terms or conditions.
