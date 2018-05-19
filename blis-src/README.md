# Blis-sys

This crate provides BLAS and/or CBLAS function using [BLIS](https://github.com/flame/blis).

Features:

* `cblas`: includes cblas binding (on by default)
* `static`: prefer static link (be very careful with this one on Apple platforms)
* `system`: do not compile blis, link it from a system-wide installation instead

It does not provides the BLAS or CBLAS functions Rust declarations. It is meant
to use the ones provides by `blas-sys` and `cblas-sys` crates instead.

See also [blas example](tests/blas_gemm.rs) or [cblas example](tests/cblas_gemm.rs).
