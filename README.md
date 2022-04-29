# blis-src [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Actions Status][actions-img]][actions-url]

This repository contains a Rust package to build and link [BLIS], the BLAS-like Library Instantiation Framework. It can be [used][usage] as a [BLAS] implementation via (Fortran) BLAS and/or CBLAS interfaces from [blas-sys] and [cblas-sys] respectively. Users simply seeking a fast BLAS are encouraged to use [blas-src] with the following in `Cargo.toml`: 

 ```toml
[dependencies]
blas-src = { version = "0.8", features = ["blis"] }
```

To access the full BLIS API, direct use of `extern "C"` is necessary at this time.

The following Cargo features are supported:

* `cblas` to build the CBLAS interface (enabled by default),
* `static` to link `libblis.a` statically,
* exactly one of `pthreads`, `openmp`, `serial` to specify the threading mode (`pthreads` by default), and
* `system` do not compile BLIS and instead use a system-provided version (must be in system's default link path).

The `system` feature is convenient after `apt install libblis-dev` or `brew install blis`, for example. When `system` is disabled, BLIS is built to use run-time dispatch to microkernels optimized for the target architecture; a single binary is thus portable and optimized for all x86-64 CPUs.

## Cross compilation

Use the `--target` option to `cargo build` and set the environment variables `TARGET_CC`, `TARGET_FC` (to detect Fortran calling convention), `TARGET_AR`, `TARGET_RANLIB`, `TARGET_CFLAGS`, and `TARGET_LDFLAGS`.
BLIS does not currently support runtime CPU detection on ARM and PowerPC targets, so you must set `BLIS_CONFNAME` to a suitable value for the target (e.g., `cortexa57` or `power9`; see [BLIS docs](https://github.com/flame/blis/blob/master/docs/BuildSystem.md#step-1-choose-a-framework-configuration)).

# License

## Apache 2.0/MIT

All original work licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
     at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[blas]: https://en.wikipedia.org/wiki/BLAS
[blas-src]: https://lib.rs/crates/blas-src
[blas-sys]: https://lib.rs/crates/blas-sys
[blis]: https://github.com/flame/blis
[cblas-sys]: https://lib.rs/crates/cblas-sys
[usage]: https://blas-lapack-rs.github.io/usage

[package-img]: https://img.shields.io/crates/v/blis-src.svg
[package-url]: https://crates.io/crates/blis-src
[documentation-img]: https://docs.rs/blis-src/badge.svg
[documentation-url]: https://docs.rs/blis-src
[actions-img]: https://github.com/blas-lapack-rs/blis-src/workflows/Rust/badge.svg
[actions-url]: https://github.com/blas-lapack-rs/blis-src/actions
