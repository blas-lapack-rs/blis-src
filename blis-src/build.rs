extern crate flate2;
extern crate git2;
extern crate reqwest;
extern crate tar;

const BLIS_VERSION:&str = "0.3.2";

use std::{fs, path, process};

fn env(k: &str) -> Option<String> {
    println!("cargo:rerun-if-env-changed={}", k);
    let v = match std::env::var(k) {
        Ok(v) => Some(v),
        Err(_) => None,
    };
    println!(" * {}={:?}", k, v);
    v
}

fn obtain_source(out_dir:&path::Path) -> path::PathBuf {
    if let Some(p) = env("BLIS_SRC_SRC_PATH") {
        p.into()
    } else {
        let blis_build = out_dir.join("src");
        if !blis_build.exists() {
            if env("BLIS_SRC_GIT").is_some() || env("BLIS_SRC_GIT_BRANCH").is_some() {
                git_clone(out_dir);
            } else {
                download_release(out_dir);
            }
        }
        blis_build
    }
}

fn git_clone(out_dir:&path::Path) {
    let mut repo = git2::build::RepoBuilder::new();
    if let Some(b) = env("BLIS_SRC_GIT_BRANCH") {
        repo.branch(&b);
    }
    let repo_url = if let Some(url) = env("BLIS_SRC_GIT_URL") {
        url
    } else {
        "https://github.com/flame/blis".to_string()
    };
    repo.clone(&*repo_url, &out_dir.join("src")).unwrap();
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

fn run(command: &mut process::Command) {
    println!("Running {:?}", command);
    assert!(command.status().unwrap().success())
}

pub fn create_shim<P: AsRef<path::Path>>(shim:P, shell: &str) {
    use std::io::Write;
    let mut shim = shim.as_ref().to_path_buf();
    fs::create_dir_all(&shim.parent().unwrap()).unwrap();
    if cfg!(target_os = "windows") {
        shim.set_extension("bat");
    };
    {
        let mut shim = fs::File::create(&shim).unwrap();
        if !cfg!(target_os = "windows") {
            writeln!(shim, "#!/bin/sh").unwrap();
        }
        shim.write_all(shell.as_bytes()).unwrap();
        writeln!(shim, "\n").unwrap();
    }
    if !cfg!(target_os = "windows") {
        fs::set_permissions(&shim, std::os::unix::fs::PermissionsExt::from_mode(0o777)).unwrap();
    }
}

fn compile(out_dir: &path::Path, src: &path::Path, only_static:bool) {
    let mut configure = process::Command::new(src.join("configure"));
    configure.current_dir(src)
            .arg(format!("--prefix={}", out_dir.to_string_lossy()))
            .arg("--enable-cblas")
            .arg("--enable-threading=pthreads");
    if only_static {
        configure.arg("--disable-shared");
    } else {
        configure.arg("--enable-shared");
    }
    configure.arg("--enable-verbose-make");
    if let Some(cc) = env("TARGET_CC") {
        configure.arg(format!("CC={}", cc));
    }
    if let Some(ranlib) = env("TARGET_RANLIB") {
        configure.arg(format!("RANLIB={}", ranlib));
    }
    let rust_arch = env("CARGO_CFG_TARGET_ARCH").unwrap();
    let arch = if let Some(arch) = env("BLIS_SRC_ARCH_OVERRIDE") {
        arch
    } else if env("TRAVIS").is_some() {
        "generic".to_string()
    } else {
        match &*rust_arch {
            "x86_64" => "x86_64",
            "arm"|"armv7" => "arm32",
            "aarch64" => "arm64",
            _ => "generic"
        }.to_string()
    };
    configure.arg(&*arch);
    run(&mut configure);
    run(process::Command::new("make")
            .arg("-j").arg("1")
            .arg("install")
            .current_dir(src))
}

fn main() {
    let out_dir = path::PathBuf::from(env("OUT_DIR").unwrap());
    let only_static = env("CARGO_FEATURE_STATIC").is_some() || env("BLIS_SRC_OVERRIDE_STATIC").is_some();
    if env("CARGO_FEATURE_SYSTEM").is_none() {
        let lib_dir = out_dir.join("lib");
        let lib = lib_dir.join("libblis.a");
        if !lib.exists() {
            let src = obtain_source(&out_dir);
            compile(&out_dir, &src, only_static);
        }
        println!("cargo:rustc-link-search=native={}", lib_dir.to_string_lossy());
    }
    println!("cargo:rustc-link-lib={}=blis", if only_static { "static" } else { "dylib" });
}
