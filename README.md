# Blis Rust bindings

This repository (will) hold crates to integrates Blis BLAS library in Rust.

* `blis-src` is the first and at this point only one. It is just a Blis
building and linking crate to use standard BLAS or C/BLAS apis.

# Roadmap

* add `blis-sys` bindings to use the more flexible api provided on top of 
which BLAS is implemented in BLIS
* add `blis` for a safe, Rust-y wrapping on top of BLIS regular and/or
object-oriented API

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
