extern crate flate2;
extern crate git2;
extern crate reqwest;
extern crate tar;

const BLIS_VERSION:&str = "0.3.2";

use std::{fs, path, process};

fn env(k: &str) -> Option<String> {
    match std::env::var(k) {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

fn obtain_source(out_dir:&path::Path) {
    let blis_build = out_dir.join("src");
    if !blis_build.exists() {
        if env("BLIS_SYS_FROM_GITHUB").is_some() {
            git_clone(out_dir);
        } else {
            download_release(out_dir);
        }
    }
}

fn git_clone(out_dir:&path::Path) {
    git2::Repository::clone("https://github.com/flame/blis", out_dir.join("src")).unwrap();
}

fn download_release(out_dir:&path::Path) {
    let blis_release = format!("https://github.com/flame/blis/archive/{}.tar.gz", BLIS_VERSION);
    let stream = reqwest::get(&blis_release).unwrap();
    let stream = flate2::read::GzDecoder::new(stream);
    tar::Archive::new(stream).unpack(&out_dir).unwrap();
    fs::rename(
        out_dir.join(format!("blis-{}", BLIS_VERSION)),
        out_dir.join("src"),
    ).unwrap();
}

fn compile(out_dir: &path::Path) {
    let blis_build = out_dir.join("src");
    let mut configure = process::Command::new(blis_build.join("configure"));
    configure.current_dir(&blis_build)
            .arg(format!("--prefix={}", out_dir.to_string_lossy()))
            .arg("--enable-cblas")
            .arg("--enable-shared")
            .arg("--enable-threading=pthreads");
    if let Some(cc) = env("TARGET_CC") {
        configure.arg(format!("CC={}", cc));
    }
    if let Some(ranlib) = env("TARGET_RANLIB") {
        configure.arg(format!("RANLIB={}", ranlib));
    }
    let arch = match &*env("CARGO_CFG_TARGET_ARCH").unwrap() {
        "x86_64" => "x86_64",
        "arm"|"armv7" => "arm32",
        "aarch64" => "arm64",
        _ => "generic"
    };
    configure.arg(arch);
    assert!(configure
            .status()
            .unwrap()
            .success()
    );
    assert!(
        process::Command::new("make")
            .arg("-j").arg("8")
            .arg("install")
            .current_dir(&blis_build)
            .status()
            .unwrap()
            .success()
    );
}

fn main() {
    let out_dir = path::PathBuf::from(env("OUT_DIR").unwrap());
    if env("CARGO_FEATURE_SYSTEM").is_none() {
        let lib_dir = out_dir.join("lib");
        let lib = lib_dir.join("libblis.a");
        if !lib.exists() {
            obtain_source(&out_dir);
            compile(&out_dir);
        }
        println!("cargo:rustc-link-search=native={}", lib_dir.to_string_lossy());
    }
    let kind = if env("CARGO_FEATURE_STATIC").is_some() { "static" } else { "dylib" };
    println!("cargo:rustc-link-lib={}=blis", kind);
}
