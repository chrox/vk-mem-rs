#[cfg(feature = "generate_bindings")]
extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut build = cc::Build::new();

    build.include("vendor/src");
    build.include("wrapper");
    build.include("wrapper/vulkan");

    // Add the files we build
    let source_files = ["wrapper/vma_lib.cpp"];

    for source_file in &source_files {
        build.file(&source_file);
    }

    let target = env::var("TARGET").unwrap();
    if target.contains("darwin") {
        build
            .flag("-std=c++11")
            .cpp_link_stdlib("c++")
            .cpp_set_stdlib("c++")
            .cpp(true);
    } else if target.contains("linux") {
        build.flag("-std=c++11").cpp_link_stdlib("stdc++").cpp(true);
    }

    build.compile("vma_cpp");

    link_vulkan();
    generate_bindings("gen/bindings.rs");
}

fn link_vulkan() {
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        println!("cargo:rustc-link-lib=dylib=vulkan-1");

        if let Ok(vulkan_sdk) = env::var("VULKAN_SDK") {
            let mut vulkan_sdk_path = PathBuf::from(vulkan_sdk);

            if target.contains("x86_64") {
                vulkan_sdk_path.push("Lib");
            } else {
                vulkan_sdk_path.push("Lib32");
            }

            println!(
                "cargo:rustc-link-search=native={}",
                vulkan_sdk_path.to_str().unwrap()
            );
        }
    } else {
        println!("cargo:rustc-link-lib=dylib=vulkan");
    }
}

#[cfg(feature = "generate_bindings")]
fn generate_bindings(output_file: &str) {
    let bindings = bindgen::Builder::default()
        .clang_arg("-I./wrapper")
        .header("vendor/src/vk_mem_alloc.h")
        .rustfmt_bindings(true)
        .blacklist_type("__darwin_.*")
        .whitelist_function("vma.*")
        .trust_clang_mangling(false)
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings!");

    bindings
        .write_to_file(std::path::Path::new(output_file))
        .expect("Unable to write bindings!");
}

#[cfg(not(feature = "generate_bindings"))]
fn generate_bindings(_: &str) {}