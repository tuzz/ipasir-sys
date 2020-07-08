## ipasir-sys

[![Build Status](https://travis-ci.org/tuzz/ipasir-sys.svg?branch=master)](https://travis-ci.org/tuzz/ipasir-sys)
[![Latest version](https://img.shields.io/crates/v/ipasir-sys.svg)](https://crates.io/crates/ipasir-sys)
[![Rust Version](https://img.shields.io/badge/rust-2018%20edition-yellow.svg)](https://rust-lang-nursery.github.io/edition-guide/editions/index.html)
[![License](https://img.shields.io/github/license/mashape/apistatus.svg)](https://github.com/tuzz/ipasir-sys/blob/master/LICENSE)

A Rust crate that contains FFI bindings for IPASIR-compatible SAT solvers.

This crate exposes the
[minimal low-level C interface](https://github.com/biotomas/ipasir/blob/master/ipasir.h)
to Rust. No more, no less. It does not try to provide safe wrappers or high
level abstractions. Those things can be built on top of this crate which is
inline with the `*-sys` naming convention as discussed in
[this article](https://kornel.ski/rust-sys-crate). Alternatively, you can use
the [ipasir-rs](https://github.com/Robbepop/ipasir-rs) crate which does provide
these things.

This crate will helpfully try to build
[Cadical](https://github.com/arminbiere/cadical) if no solver is specified and
has integration tests to verify the bindings work.

## What is IPASIR?

IPASIR is a standard interface for incremental SAT solvers. It is the reverse
acronym for _Re-entrant Incremental Satisfiability Application Program Interface_
and was introduced at the 2015 annual
[SAT competition](https://www.cs.helsinki.fi/u/mjarvisa/papers/jarvisalo-leberre-roussel-simon.aimag.pdf).

More explanation can be found in section 6.2 of
[this paper](http://fmv.jku.at/papers/BalyoBiereIserSinz-AI-16.pdf).

## How to use this crate

There are two ways to use this crate:

1. You can provide your own static library of a solver that implements IPASIR
2. You can do nothing and the crate will try to build and link Cadical

The end result is the same. The IPASIR functions can be called by wrapping them
in unsafe blocks:

```rust
use ipasir_sys::*;

fn main() {
    unsafe {
      let solver = ipasir_init();

      ipasir_add(solver, 1);
      ipasir_add(solver, 0);

      let sat_status = ipasir_solve(solver);
      assert_eq!(sat_status, 10);
    }
}
```

For a more comprehensive example, see
[coloring_test.rs](https://github.com/tuzz/ipasir-sys/blob/master/tests/coloring_test.rs)
or refer to
[interface_test.rs](https://github.com/tuzz/ipasir-sys/blob/master/tests/interface_test.rs).

## Providing your own library

You can provide your own static library by setting the `IPASIR` environment
variable at build time:

```sh
$ IPASIR=/path/to/libsolver.a cargo build
```

The crate will copy the library to its build directory and try to link against
it. You must use an absolute path but the library's name does not matter. If
your library has other dependencies it is recommended you inline them into your
static library. Alternatively, you can try to pass additional link flags to
cargo.

## Using Cadical

If the `IPASIR` environment variable is not set, the crate will try to compile a
version of Cadical that is vendored as part of the crate. If this doesn't work,
please try cloning this crate from GitHub and running `cargo test` on its own.
It may also be helpful to refer to the [Travis CI](https://travis-ci.org/tuzz/ipasir-sys)
build and [its configuration](.travis.yml).

If you can't get it work, please [open an issue](https://github.com/tuzz/ipasir-sys/issues/new).

## Ideas for improvement

- Vendor more solvers and switch between them easily
- Improve operating system support (e.g. Windows)
- Add automated tests against different solvers and platforms

## License

This crate has the
[MIT License](https://github.com/tuzz/ipasir-sys/blob/master/LICENSE) but please
check the license restrictions of the vendored software before using it.
