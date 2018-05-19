extern crate blis_src;
extern crate blas_sys;

#[test]
fn f_gemm() {
    let a = [1.0];
    let b = [2.0];
    let mut c = [12.0];

    unsafe {
        blas_sys::sgemm_(
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

