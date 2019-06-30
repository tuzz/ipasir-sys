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
your library has other dependencies, you can either pass additional flags to
cargo or inline them into your static library.

## Using the default solver

If the `IPASIR` environment variable is not set, the crate will try to compile a
version of Cadical which is vendored as part of the crate. This compilation
depends on a relatively modern version of C++ and could fail if its standard
library cannot be found (libstdc++). At time of writing, the
[clang](https://clang.llvm.org/) version that ships with MacOS fails to compile
Cadical.

To fix this, there are two _escape hatches_ that can be used to aid compilation:

1. You can explicitly tell it which compiler to use by symlinking it to `/usr/local/bin/g++`
2. You can ensure libstdc++ is discoverable by symlinking it to `/usr/local/lib/libstdc++.a`

You may also find the crate compiles but fails at runtime due to missing
linker symbols. This is likely the same problem and you should try re-compiling,
following the instructions above. It may be helpful to clone this crate and
build it on its own with `cargo test` or refer to the
[Travis CI](https://travis-ci.org/tuzz/ipasir-sys) build and
[its configuration](.travis.yml).

## Compiling on MacOS

The [GNU Compiler Collection](https://gcc.gnu.org/) is able to compile Cadical
on MacOS. Based on the section above, here's how to fix it:

```sh
$ brew install gcc

$ ln -s /usr/local/Cellar/gcc/*/lib/gcc/*/libstdc++.a /usr/local/lib/
$ ln -s /usr/local/Cellar/gcc/*/bin/g++-* /usr/local/bin/g++

$ cargo build
```

If this still fails, or if you're unable to compile the crate on linux, please
[open an issue](https://github.com/tuzz/ipasir-sys/issues/new).

## Ideas for improvement

- Vendor more solvers and switch between them with a [crate feature](https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section)
- Improve operating system support (e.g. Windows)
- Add automated tests against different solvers and platforms

## License

This crate has the
[MIT License](https://github.com/tuzz/ipasir-sys/blob/master/LICENSE) but please
check the license restrictions of the vendored software before using it.
