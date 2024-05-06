# blis-src

This crate provides BLAS and/or CBLAS functions using [BLIS](https://github.com/flame/blis).

## Features:

* `cblas`: build the CBLAS interface (enabled by default)
* `static`: prefer static link (be very careful with this one on Apple platforms)
* `pthreads` or `openmp` or `serial`: choose exactly one to specify the threading mode (`pthreads` by default)
* `system`: do not compile BLIS and instead use a system-provided version (must be in system's default link path).

This package does not provides Rust declarations for BLAS or CBLAS, which
are available in the [`blas-sys`](https://lib.rs/crates/blas-sys) and
[`cblas-sys`](https://lib.rs/crates/cblas-sys) crates. See the [blas
example](https://github.com/blas-lapack-rs/blis-src/blob/main/blis-src/tests/blas_gemm.rs) and [cblas example](https://github.com/blas-lapack-rs/blis-src/blob/main/blis-src/tests/cblas_gemm.rs)
for usage.

Users simply seeking a fast BLAS are encouraged to use
[`blas-sys`](https://lib.rs/crates/blas-src) with the following in
`Cargo.toml`:

```toml
[dependencies]
blas-src = { version = "0.10", features = ["blis"] }
```
