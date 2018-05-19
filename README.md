# Blis Rust bindings

This repository (will) hold crates to integrates Blis BLAS library in Rust.

* `blis-src` is the first and at this point only one. It is just a Blis
building and linking crate to use standard BLAS or C/BLAS apis.

# Roadmap

* add `blis-sys` bindings to use the more flexible api provided on top of 
which BLAS is implemented in BLIS
* add `blis` for a safe, Rust-y wrapping on top of BLIS regular and/or
object-oriented API
