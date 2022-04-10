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
        (None, None, None) => panic!(
            "One of the following features must be enabled: 'pthreads', 'openmp', and 'serial'."
        ),
        _ => panic!("Features 'pthreads', 'openmp', and 'serial' are mutually exclusive."),
    };
    configure.arg(format!("--enable-threading={}", threading));
    if env("CARGO_FEATURE_STATIC").is_some() {
        configure.args(&["--enable-static", "--disable-shared"]);
    } else {
        configure.args(&["--disable-static", "--enable-shared"]);
    }
    for var in &["CC", "FC", "RANLIB", "AR", "CFLAGS", "LDFLAGS"] {
        if let Some(value) = env(&format!("TARGET_{}", var)) {
            configure.arg(format!("{}={}", var, value));
        }
    }
    let rust_arch = env("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = env("CARGO_CFG_TARGET_OS").unwrap();
    let blis_confname = if let Some(a) = env("BLIS_CONFNAME") {
        a
    } else {
        match (&*target_os, &*rust_arch) {
            (_, "x86_64") => "x86_64", // Build all microkernels; run-time dispatch

            // BLIS does not have run-time arch detection on ARM or PowerPC.
            // We'll let BLIS configure determine the best match.
            (_, "arm" | "armv7") => "auto",      // cortexa9/cortexa15
            ("macos", "aarch64") => "firestorm", // Apple M1
            (_, "aarch64") => "auto",            // cortexa57/thunderx2
            (_, "powerpc64") => "auto",          // bgq/power9/power10
            _ => "generic",
        }
        .to_string()
    };
    configure.arg(blis_confname);
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
            // Check if upstream is a non-empty directory.
            if std::fs::read_dir("upstream")
                .ok()
                .and_then(|mut d| d.next().filter(|de| de.is_ok()))
                .is_none()
            {
                panic!("upstream directory can not be read. Consider running `git submodule update --init`.");
            }
            run(Command::new("cp").arg("-R").arg("upstream").arg(&build_dir));
            compile(&build_dir, &out_dir);
        }
        println!(
            "cargo:rustc-link-search=native={}",
            lib_dir.to_string_lossy()
        );
        let include_dir = out_dir.join("include");
        println!("cargo:include={}", include_dir.to_string_lossy());
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
