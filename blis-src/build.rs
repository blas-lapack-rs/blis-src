use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn env(k: &str) -> Option<String> {
    match std::env::var(k) {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

fn compile(blis_build: &Path, out_dir: &Path) {
    let mut configure = Command::new(blis_build.join("configure"));
    configure
        .current_dir(&blis_build)
        .arg(format!("--prefix={}", out_dir.to_string_lossy()))
        .arg("--enable-cblas");
    let threading = match (
        env("CARGO_FEATURE_PTHREADS"),
        env("CARGO_FEATURE_OPENMP"),
        env("CARGO_FEATURE_SERIAL"),
    ) {
        (Some(_), None, None) => "pthreads",
        (None, Some(_), None) => "openmp",
        (None, None, Some(_)) => "no",
        _ => panic!("Features 'pthreads', 'openmp', and 'serial' are mutually exclusive."),
    };
    configure.arg(format!("--enable-threading={}", threading));
    if env("CARGO_FEATURE_STATIC").is_some() {
        configure.args(&["--enable-static", "--disable-shared"]);
    } else {
        configure.args(&["--disable-static", "--enable-shared"]);
    }
    if let Some(cc) = env("TARGET_CC") {
        configure.arg(format!("CC={}", cc));
    }
    if let Some(ranlib) = env("TARGET_RANLIB") {
        configure.arg(format!("RANLIB={}", ranlib));
    }
    let rust_arch = env("CARGO_CFG_TARGET_ARCH").unwrap();
    let arch = if env("TRAVIS").is_some() {
        "generic"
    } else {
        match &*rust_arch {
            "x86_64" => "x86_64", // Build all microkernels; run-time dispatch

            // BLIS does not have run-time arch detection on ARM or PowerPC.
            // We'll let BLIS configure determine the best match.
            "arm" | "armv7" => "auto", // cortexa9/cortexa15
            "aarch64" => "auto",       // cortexa57/thunderx2
            "powerpc64" => "auto",     // bgq/power9/power10
            _ => "generic",
        }
    };
    configure.arg(arch);
    run(&mut configure);
    let makeflags = env("CARGO_MAKEFLAGS").unwrap();
    run(Command::new("make")
        .arg("install")
        .env("MAKEFLAGS", makeflags)
        .current_dir(&blis_build));
}

fn main() {
    let out_dir = PathBuf::from(env("OUT_DIR").unwrap());
    if env("CARGO_FEATURE_SYSTEM").is_none() {
        let lib_dir = out_dir.join("lib");
        let lib = lib_dir.join("libblis.a");
        if !lib.exists() {
            let target = env("TARGET").unwrap();
            let build_dir = out_dir.join(format!("blis_{}", target.to_lowercase()));
            if build_dir.exists() {
                fs::remove_dir_all(&build_dir).unwrap();
            }
            if !std::fs::metadata("upstream").is_ok() {
                panic!("upstream directory can not be read. Consider running `git submodule update --init`.");
            }
            run(Command::new("cp").arg("-R").arg("upstream").arg(&build_dir));
            compile(&build_dir, &out_dir);
        }
        println!(
            "cargo:rustc-link-search=native={}",
            lib_dir.to_string_lossy()
        );
    }
    let kind = if env("CARGO_FEATURE_STATIC").is_some() {
        "static"
    } else {
        "dylib"
    };
    println!("cargo:rustc-link-lib={}=blis", kind);
    println!("cargo:rerun-if-changed=build.rs");
}

fn run(command: &mut Command) {
    println!("Running: `{:?}`", command);
    assert!(command.status().unwrap().success());
}
