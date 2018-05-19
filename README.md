# Blis-sys

This crate provides BLAS and/or CBLAS function using [BLIS](https://github.com/flame/blis).

Features:

* `cblas`: includes cblas binding (on by default)
* `static`: prefer static link (be very careful with this one on Apple platforms)
* `system`: do not compile blis, link it from a system-wide installation instead


