//! # blis-src
//!
//! This crate provides BLAS and/or CBLAS function using [BLIS](https://github.com/flame/blis).
//!
//! Features:
//!
//! * `cblas`: build the CBLAS interface (enabled by default)
//! * `static`: prefer static link (be very careful with this one on Apple platforms)
//! * `pthreads` or `openmp` or `serial`: choose exactly one to specify the threading mode (`pthreads` by default)
//! * `system`: do not compile BLIS and instead use a system-provided version (must be in system's default link path).
//!
//! This package does not provides Rust declarations for BLAS or CBLAS, which
//! are available in the [`blas-sys`](https://lib.rs/crates/blas-sys) and
//! [`cblas-sys`](https://lib.rs/crates/cblas-sys) crates. See the [blas
//! example](../tests/blas_gemm.rs) and [cblas example](../tests/cblas_gemm.rs)
//! for usage.
//!
//! Users simply seeking a fast BLAS are encouraged to use
//! [`blas-sys`](https://lib.rs/crates/blas-src) with the following in
//! `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! blas-src = { version = "0.7", features = ["blis"] }
//! ```

#[cfg(test)]
mod tests {

    extern crate libc;
    use self::libc::*;

    extern "C" {
        pub fn bli_info_get_version_str() -> *const c_char;
        pub fn sgemm_(
            transa: *const c_char,
            transb: *const c_char,
            m: *const c_int,
            n: *const c_int,
            k: *const c_int,
            alpha: *const c_float,
            a: *const c_float,
            lda: *const c_int,
            b: *const c_float,
            ldb: *const c_int,
            beta: *const c_float,
            c: *mut c_float,
            ldc: *const c_int,
        );
    }

    #[test]
    fn it_links() {
        use std::ffi::CStr;
        unsafe {
            let s = CStr::from_ptr(bli_info_get_version_str());
            println!("blis version: {:?}", s);
        }
    }

    #[test]
    fn sgemm() {
        let a = [1.0];
        let b = [2.0];
        let mut c = [12.0];

        unsafe {
            sgemm_(
                &(b'N' as i8),
                &(b'N' as i8),
                &1,
                &1,
                &1,
                &1.0,
                a.as_ptr(),
                &1,
                b.as_ptr(),
                &1,
                &0.0,
                c.as_mut_ptr(),
                &1,
            );
        }
        assert_eq!(&c, &[2.0]);
    }
}
