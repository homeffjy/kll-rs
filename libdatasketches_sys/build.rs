extern crate bindgen;
extern crate cc;
extern crate cmake;

use cc::Build;
use cmake::Config;
use std::path::{Path, PathBuf};
use std::{env, str};

// Generate the bindings to datasketches C-API.
fn bindgen_datasketches(file_path: &Path) {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .ctypes_prefix("libc")
        .generate()
        .expect("unable to generate datasketches bindings");

    bindings
        .write_to_file(file_path)
        .expect("unable to write datasketches bindings");
}

// Determine if need to update bindings
fn config_binding_path() {
    let file_path: PathBuf;

    let target = env::var("TARGET").unwrap_or_else(|_| "".to_owned());
    match target.as_str() {
        "x86_64-unknown-linux-gnu" | "aarch64-unknown-linux-gnu" => {
            file_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
                .join("bindings")
                .join(format!("{}-bindings.rs", target));
            if env::var("UPDATE_BIND")
                .map(|s| s.as_str() == "1")
                .unwrap_or(false)
            {
                bindgen_datasketches(&file_path);
            }
        }
        _ => {
            file_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("datasketches-bindings.rs");
            bindgen_datasketches(&file_path);
        }
    };
    println!(
        "cargo:rustc-env=BINDING_PATH={}",
        file_path.to_str().unwrap()
    );
}

fn main() {
    println!("cargo:rerun-if-env-changed=UPDATE_BIND");

    let mut build = build_datasketches();
    
    build.cpp(true).file("wrapper.cpp");
    if env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        build.flag("-std=c++14");
    }
    link_cpp(&mut build);
    build.warnings(false).compile("libdatasketches.a");
    
    config_binding_path();
}

fn link_cpp(build: &mut Build) {
    let tool = build.get_compiler();
    let stdlib = if tool.is_like_gnu() {
        "libstdc++.a"
    } else if tool.is_like_clang() {
        "libc++.a"
    } else {
        // Don't link to c++ statically on windows.
        return;
    };
    
    let output = tool
        .to_command()
        .arg("--print-file-name")
        .arg(stdlib)
        .output()
        .unwrap();
    if !output.status.success() || output.stdout.is_empty() {
        // fallback to dynamically
        return;
    }
    
    let path = match str::from_utf8(&output.stdout) {
        Ok(path) => PathBuf::from(path),
        Err(_) => return,
    };
    if !path.is_absolute() {
        return;
    }
    
    // remove lib prefix and .a postfix.
    let libname = &stdlib[3..stdlib.len() - 2];
    // optional static linking
    if cfg!(feature = "static") {
        println!("cargo:rustc-link-lib=static={}", &libname);
    } else {
        println!("cargo:rustc-link-lib=dylib={}", &libname);
    }
    println!(
        "cargo:rustc-link-search=native={}",
        path.parent().unwrap().display()
    );
    build.cpp_link_stdlib(None);
}

fn build_datasketches() -> Build {
    // Option 1: Use cmake to build datasketches-cpp if it has CMakeLists.txt
    // Option 2: Build directly with cc if we have individual cpp files
    
    let cur_dir = env::current_dir().unwrap();
    let mut build = Build::new();
    
    // Include datasketches headers
    build.include(cur_dir.join("datasketches-cpp").join("common").join("include"));
    
    // Add datasketches source files directly
    let datasketches_src = cur_dir.join("datasketches-cpp");
    if datasketches_src.exists() {
        // Add KLL-specific source files
        build.file(datasketches_src.join("kll.cpp"));
    }
    
    build
}
