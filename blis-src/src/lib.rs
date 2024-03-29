#![doc = include_str!("../README.md")]

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
