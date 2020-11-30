extern crate blis_src;
extern crate cblas_sys;

#[test]
fn c_gemm() {
    let a = [1.0];
    let b = [2.0];
    let mut c = [12.0];

    unsafe {
        cblas_sys::cblas_sgemm(
            cblas_sys::CBLAS_LAYOUT::CblasColMajor,
            cblas_sys::CBLAS_TRANSPOSE::CblasNoTrans,
            cblas_sys::CBLAS_TRANSPOSE::CblasNoTrans,
            1,
            1,
            1,
            1.0,
            a.as_ptr(),
            1,
            b.as_ptr(),
            1,
            0.0,
            c.as_mut_ptr(),
            1,
        );
    }
    assert_eq!(&c, &[2.0]);
}
